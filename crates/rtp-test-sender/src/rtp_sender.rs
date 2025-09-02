use crate::Args;
use crate::test_audio::AudioGenerator;
use anyhow::Result;
use rand::Rng;
use rtp::packet::Packet;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::time;
use tracing::{debug, info};
use webrtc_util::marshal::Marshal;

pub struct RtpSender {
    socket: UdpSocket,
    target: SocketAddr,
    args: Args,
    ssrc: u32,
    sequence_number: u16,
    timestamp: u32,
    audio_generator: AudioGenerator,
}

impl RtpSender {
    pub async fn new(args: Args) -> Result<Self> {
        let bind_addr: SocketAddr = format!("0.0.0.0:{}", args.port).parse()?;
        let socket = UdpSocket::bind(bind_addr).await?;
        let local_addr = socket.local_addr()?;
        info!("RTP sender bound to {}", local_addr);

        let mut rng = rand::thread_rng();
        let ssrc: u32 = rng.r#gen();
        info!("Using SSRC: {}", ssrc);

        let sample_rate = if args.payload_type == 111 {
            48000
        } else {
            8000
        };
        let channels = 1; // Start with mono
        let audio_generator = AudioGenerator::new(sample_rate, channels);

        Ok(Self {
            socket,
            target: args.target,
            args,
            ssrc,
            sequence_number: rng.r#gen(),
            timestamp: rng.r#gen(),
            audio_generator,
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    pub async fn run(&mut self) -> Result<()> {
        let start_time = Instant::now();
        let duration = Duration::from_secs(self.args.duration);
        let mut interval = time::interval(Duration::from_millis(self.args.interval));
        let mut rng = rand::thread_rng();

        let mut packets_sent = 0u64;
        let mut packets_dropped = 0u64;
        let mut packets_reordered = 0u64;
        let mut last_swap_time = Instant::now();

        // Buffer for out-of-order simulation
        let mut packet_buffer: Option<Vec<u8>> = None;

        while start_time.elapsed() < duration {
            interval.tick().await;

            // Check if we should swap channels
            if self.args.swap_channels_after > 0
                && last_swap_time.elapsed() >= Duration::from_secs(self.args.swap_channels_after)
            {
                self.audio_generator.swap_channels();
                last_swap_time = Instant::now();
            }

            // Generate audio samples
            let samples_per_packet = (self.args.interval * 8) as usize; // 8 samples per ms for 8kHz
            let audio_data = if self.args.payload_type == 0 {
                self.audio_generator
                    .generate_pcmu_samples(samples_per_packet)
            } else {
                // For demo, just use raw PCM for "Opus"
                self.audio_generator.generate_samples(samples_per_packet)
            };

            // Create RTP packet
            let packet = Packet {
                header: rtp::header::Header {
                    version: 2,
                    padding: false,
                    extension: false,
                    marker: false,
                    payload_type: self.args.payload_type,
                    sequence_number: self.sequence_number,
                    timestamp: self.timestamp,
                    ssrc: self.ssrc,
                    csrc: vec![],
                    extension_profile: 0,
                    extensions: vec![],
                    extensions_padding: 0,
                },
                payload: audio_data.into(),
            };

            let packet_bytes = packet.marshal()?;
            let packet_data = packet_bytes.to_vec();

            // Simulate packet loss
            if self.args.packet_loss > 0 && rng.gen_range(0..100) < self.args.packet_loss {
                debug!("Dropping packet (seq: {})", self.sequence_number);
                packets_dropped += 1;
            } else if self.args.out_of_order > 0 && rng.gen_range(0..100) < self.args.out_of_order {
                // Simulate out-of-order packets
                if let Some(buffered) = packet_buffer.take() {
                    // Send the buffered packet (out of order)
                    self.socket.send_to(&buffered, self.target).await?;
                    debug!("Sent buffered packet out of order");
                    packets_reordered += 1;
                }
                // Buffer current packet for later
                packet_buffer = Some(packet_data);
                debug!("Buffering packet (seq: {})", self.sequence_number);
            } else {
                // Normal send
                self.socket.send_to(&packet_data, self.target).await?;
                packets_sent += 1;

                // Send any buffered packet
                if let Some(buffered) = packet_buffer.take() {
                    self.socket.send_to(&buffered, self.target).await?;
                    packets_reordered += 1;
                }
            }

            // Update sequence and timestamp
            self.sequence_number = self.sequence_number.wrapping_add(1);
            self.timestamp = self
                .timestamp
                .wrapping_add((samples_per_packet * self.args.interval as usize / 20) as u32);

            if packets_sent % 50 == 0 && packets_sent > 0 {
                debug!(
                    "Sent {} packets, dropped {}, reordered {}",
                    packets_sent, packets_dropped, packets_reordered
                );
            }
        }

        // Send any remaining buffered packet
        if let Some(buffered) = packet_buffer.take() {
            self.socket.send_to(&buffered, self.target).await?;
            packets_reordered += 1;
        }

        info!(
            "Transmission complete: {} packets sent, {} dropped, {} reordered",
            packets_sent, packets_dropped, packets_reordered
        );

        Ok(())
    }
}

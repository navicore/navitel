use anyhow::Result;
use bytes::BytesMut;
use rtp::packet::Packet;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use webrtc_util::marshal::Unmarshal;

use crate::stream_manager::StreamManager;
use shared_types::{AudioChunk, AudioFormat, LatencyMetadata};

const MAX_PACKET_SIZE: usize = 1500;

pub struct RtpReceiver {
    socket: Arc<UdpSocket>,
    stream_manager: Arc<RwLock<StreamManager>>,
}

impl RtpReceiver {
    pub async fn new(bind_addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        info!("RTP receiver listening on {}", bind_addr);

        Ok(Self {
            socket: Arc::new(socket),
            stream_manager: Arc::new(RwLock::new(StreamManager::new())),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let mut buf = vec![0u8; MAX_PACKET_SIZE];

        loop {
            match self.socket.recv_from(&mut buf).await {
                Ok((len, source_addr)) => {
                    let packet_data = &buf[..len];
                    if let Err(e) = self.handle_packet(packet_data, source_addr).await {
                        warn!("Failed to handle packet from {}: {}", source_addr, e);
                    }
                }
                Err(e) => {
                    error!("Failed to receive packet: {}", e);
                }
            }
        }
    }

    async fn handle_packet(&self, data: &[u8], source_addr: SocketAddr) -> Result<()> {
        let packet = Packet::unmarshal(&mut BytesMut::from(data))?;

        debug!(
            "Received RTP packet: SSRC={}, Seq={}, TS={}, PT={}",
            packet.header.ssrc,
            packet.header.sequence_number,
            packet.header.timestamp,
            packet.header.payload_type
        );

        let mut manager = self.stream_manager.write().await;
        let stream_id = manager.get_or_create_stream(source_addr, packet.header.ssrc);

        let mut metadata = LatencyMetadata::new(stream_id);
        metadata.start_stage("rtp_ingestion", "rtp-ingest");

        let audio_chunk = AudioChunk {
            data: packet.payload,
            format: detect_audio_format(packet.header.payload_type),
            sequence_number: u32::from(packet.header.sequence_number),
            timestamp: packet.header.timestamp,
            metadata: metadata.clone(),
        };

        manager.process_audio_chunk(stream_id, &audio_chunk);

        metadata.end_stage();
        debug!(
            "Processed packet from {} in {:?}",
            source_addr,
            metadata.total_latency()
        );

        Ok(())
    }
}

fn detect_audio_format(payload_type: u8) -> AudioFormat {
    match payload_type {
        0 => AudioFormat::g711_ulaw_mono(),
        111 => AudioFormat::opus_mono_48khz(),
        _ => AudioFormat {
            codec: shared_types::AudioCodec::Pcm,
            sample_rate: 8000,
            channels: 1,
            bits_per_sample: 16,
        },
    }
}

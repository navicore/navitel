mod rtp_sender;
mod test_audio;

use anyhow::Result;
use clap::Parser;
use std::net::SocketAddr;
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target address to send RTP packets to
    #[arg(short, long, default_value = "127.0.0.1:5004")]
    target: SocketAddr,

    /// Source port to bind to (0 for random)
    #[arg(short, long, default_value = "0")]
    port: u16,

    /// Payload type (0=PCMU, 111=Opus)
    #[arg(short = 'c', long, default_value = "0")]
    payload_type: u8,

    /// Duration in seconds to send
    #[arg(short, long, default_value = "10")]
    duration: u64,

    /// Packet interval in milliseconds
    #[arg(short, long, default_value = "20")]
    interval: u64,

    /// Simulate packet loss (percentage 0-100)
    #[arg(short = 'l', long, default_value = "0")]
    packet_loss: u8,

    /// Simulate out-of-order packets (percentage 0-100)
    #[arg(short = 'o', long, default_value = "0")]
    out_of_order: u8,

    /// Audio file to send (optional, uses generated tone if not specified)
    #[arg(short = 'f', long)]
    file: Option<String>,

    /// Simulate channel swap after N seconds (0 = disabled)
    #[arg(long, default_value = "0")]
    swap_channels_after: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let args = Args::parse();

    info!("Starting RTP Test Sender");
    info!("Target: {}", args.target);
    info!("Payload Type: {}", args.payload_type);
    info!("Duration: {}s", args.duration);
    info!("Packet Interval: {}ms", args.interval);

    if args.packet_loss > 0 {
        info!("Simulating {}% packet loss", args.packet_loss);
    }
    if args.out_of_order > 0 {
        info!("Simulating {}% out-of-order packets", args.out_of_order);
    }
    if args.swap_channels_after > 0 {
        info!(
            "Will swap channels after {} seconds",
            args.swap_channels_after
        );
    }

    let mut sender = rtp_sender::RtpSender::new(args).await?;
    sender.run().await?;

    info!("RTP Test Sender finished");
    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(Level::INFO.as_str())),
        )
        .init();
}

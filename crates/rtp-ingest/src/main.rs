mod rtp_receiver;
mod stream_manager;

use anyhow::Result;
use std::net::SocketAddr;
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    info!("Starting RTP Ingest Service");

    let bind_addr: SocketAddr = "0.0.0.0:5004".parse()?;
    info!("Binding to {}", bind_addr);

    let receiver = rtp_receiver::RtpReceiver::new(bind_addr).await?;
    receiver.run().await?;

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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

pub type StreamId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMetadata {
    pub id: StreamId,
    pub source_addr: SocketAddr,
    pub created_at: DateTime<Utc>,
    pub state: StreamState,
    pub codec: String,
    pub ssrc: Option<u32>,
    pub call_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamState {
    Connecting,
    Active,
    Paused,
    Disconnected,
    Error,
}

impl StreamMetadata {
    pub fn new(source_addr: SocketAddr) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_addr,
            created_at: Utc::now(),
            state: StreamState::Connecting,
            codec: String::new(),
            ssrc: None,
            call_id: None,
        }
    }

    #[must_use]
    pub fn with_rtp_info(mut self, ssrc: u32, codec: String) -> Self {
        self.ssrc = Some(ssrc);
        self.codec = codec;
        self
    }
}

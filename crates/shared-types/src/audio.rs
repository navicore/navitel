use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::latency::LatencyMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioChunk {
    pub data: Bytes,
    pub format: AudioFormat,
    pub sequence_number: u32,
    pub timestamp: u32,
    pub metadata: LatencyMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioFormat {
    pub codec: AudioCodec,
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioCodec {
    Opus,
    G711Ulaw,
    G711Alaw,
    G722,
    Pcm,
}

impl AudioFormat {
    pub const fn opus_mono_48khz() -> Self {
        Self {
            codec: AudioCodec::Opus,
            sample_rate: 48000,
            channels: 1,
            bits_per_sample: 16,
        }
    }

    pub const fn g711_ulaw_mono() -> Self {
        Self {
            codec: AudioCodec::G711Ulaw,
            sample_rate: 8000,
            channels: 1,
            bits_per_sample: 8,
        }
    }
}

use std::collections::HashMap;
use std::net::SocketAddr;
use tracing::{debug, info};
use uuid::Uuid;

use shared_types::{AudioChunk, StreamId, StreamMetadata, StreamState};

pub struct StreamManager {
    streams: HashMap<StreamId, StreamInfo>,
    ssrc_to_stream: HashMap<u32, StreamId>,
}

struct StreamInfo {
    #[allow(dead_code)]
    metadata: StreamMetadata,
    packet_count: u64,
    last_sequence: u16,
    out_of_order_count: u64,
}

impl StreamManager {
    pub fn new() -> Self {
        Self {
            streams: HashMap::new(),
            ssrc_to_stream: HashMap::new(),
        }
    }

    pub fn get_or_create_stream(&mut self, source_addr: SocketAddr, ssrc: u32) -> StreamId {
        if let Some(&stream_id) = self.ssrc_to_stream.get(&ssrc) {
            return stream_id;
        }

        let stream_id = Uuid::new_v4();
        let mut metadata = StreamMetadata::new(source_addr);
        metadata.ssrc = Some(ssrc);
        metadata.state = StreamState::Active;

        info!(
            "New RTP stream detected: ID={}, SSRC={}, Source={}",
            stream_id, ssrc, source_addr
        );

        let stream_info = StreamInfo {
            metadata,
            packet_count: 0,
            last_sequence: 0,
            out_of_order_count: 0,
        };

        self.streams.insert(stream_id, stream_info);
        self.ssrc_to_stream.insert(ssrc, stream_id);

        stream_id
    }

    pub fn process_audio_chunk(&mut self, stream_id: StreamId, chunk: &AudioChunk) {
        if let Some(stream_info) = self.streams.get_mut(&stream_id) {
            stream_info.packet_count += 1;

            #[allow(clippy::cast_possible_truncation)]
            let seq = chunk.sequence_number as u16;
            if stream_info.packet_count > 1 {
                let expected = stream_info.last_sequence.wrapping_add(1);
                if seq != expected {
                    stream_info.out_of_order_count += 1;
                    debug!(
                        "Out-of-order packet: expected {}, got {} (stream: {})",
                        expected, seq, stream_id
                    );
                }
            }
            stream_info.last_sequence = seq;

            if stream_info.packet_count % 1000 == 0 {
                info!(
                    "Stream {} stats: {} packets, {} out-of-order",
                    stream_id, stream_info.packet_count, stream_info.out_of_order_count
                );
            }

            // TODO: Forward to audio-router service
            debug!(
                "Processed audio chunk: stream={}, seq={}, size={}",
                stream_id,
                chunk.sequence_number,
                chunk.data.len()
            );
        }
    }

    #[allow(dead_code)]
    pub fn get_stream_metadata(&self, stream_id: &StreamId) -> Option<&StreamMetadata> {
        self.streams.get(stream_id).map(|info| &info.metadata)
    }
}

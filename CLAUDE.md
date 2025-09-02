# Claude Code Project Context

## Project Overview
Navitel is a learning project for Ed Sweeney (@navicore) to explore modern telephony concepts, particularly RTP audio processing and real-time latency tracking. The goal is to build Kubernetes-ready microservices that could be spun off into production tools.

## Background Context
- Ed works at a SaaS providing real-time AI audio analysis for call centers
- Use cases: speech rate detection, keyword monitoring, abuse detection
- Has 90s call center tech experience (ACD, CTI) but wants to learn modern VoIP/RTP

## Key Technical Decisions
1. **Language**: Rust 2024 edition
2. **License**: MIT only (no MPL or business licenses)
3. **Redis Alternative**: Using Valkey (Redis fork) or NATS due to Redis license change
4. **CI/CD Pattern**: Using `just` command runner with `.cargo/config.toml` for consistent clippy behavior
5. **Editor**: Neovim with rust-analyzer LSP

## Architecture Plan

### Phase 1: Foundation (Current)
- [x] Set up Rust workspace with 8 crates
- [x] Configure CI/CD with GitHub Actions
- [ ] Implement basic RTP ingestion
- [ ] Create latency metadata format

### Phase 2: Core Pipeline
- [ ] RTP packet reception and parsing
- [ ] Audio chunk buffering with timestamp metadata
- [ ] Simple passthrough to measure baseline latency
- [ ] OpenTelemetry integration for distributed tracing

### Phase 3: Processing
- [ ] Integrate Whisper or Vosk for STT
- [ ] Implement keyword detection (Aho-Corasick)
- [ ] Speech rate analyzer
- [ ] Real-time WebSocket event streaming

### Phase 4: Production Features
- [ ] Backpressure handling with circuit breakers
- [ ] Horizontal scaling with consistent hashing
- [ ] Prometheus metrics
- [ ] Kubernetes manifests/Helm charts

## Latency Tracking Design
Each audio chunk should carry metadata:
```rust
struct AudioChunk {
    data: Vec<u8>,
    metadata: ChunkMetadata,
}

struct ChunkMetadata {
    stream_id: Uuid,
    sequence: u32,
    ingestion_ts: Instant,
    processing_stages: Vec<ProcessingStage>,
}

struct ProcessingStage {
    name: String,
    start_ts: Instant,
    end_ts: Option<Instant>,
}
```

## Development Commands
```bash
just ci          # Run all checks (fmt, clippy, test) - same as CI
just watch       # Auto-run checks on file changes
just lint        # Run clippy with project settings
just test        # Run all tests
```

## Important Notes
- ALWAYS run `just ci` before commits to ensure CI will pass
- Clippy settings in `.cargo/config.toml` ensure local/CI consistency
- Main branch CI runs on push, PRs run on all branches
- Use `just` commands, not direct cargo commands, for consistency

## Next Immediate Tasks
1. Create shared-types crate with core domain models
2. Implement basic RTP ingestion in rtp-ingest crate
3. Set up OpenTelemetry tracing infrastructure
4. Create simple audio passthrough for latency baseline

## Technical Challenges to Explore
- RTP jitter buffer implementation
- Handling packet loss and reordering
- Audio codec transcoding (Opus, G.711, etc.)
- Real-time backpressure without dropping audio
- Distributed tracing across async boundaries
- **Channel swapping bugs** - Simulate and fix common telephony issues

## Channel Swapping Simulator (Future)
Ed mentioned they see "channel swapping" bugs in production where customer/agent audio gets reversed. We should build simulators for:
1. Two-party call with agent/customer channels that can be deliberately swapped
2. Packet reordering scenarios that cause channel confusion  
3. SSRC changes mid-stream (network reconnections)
4. Codec transcoding bugs that flip channels
5. Conference bridge mixing errors

This will help learn:
- Proper channel tracking through audio pipeline
- RTP sequence number and SSRC handling
- Debugging techniques for hard-to-reproduce production issues
- How stereo/mono conversions can go wrong

## Resources
- Repository: https://github.com/navicore/navitel
- RTP RFC: https://datatracker.ietf.org/doc/html/rfc3550
- WebRTC Media: https://docs.rs/webrtc-media
- NATS Streaming: https://docs.nats.io/nats-concepts/jetstream
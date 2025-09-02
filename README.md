# Navitel

Real-time telephony audio processing pipeline with latency tracking built in Rust.

## Overview

Navitel is a learning project exploring modern telephony concepts including RTP stream processing, real-time audio analysis, and distributed latency tracking. The system is designed as a collection of microservices that can be deployed to Kubernetes.

## Architecture

The project consists of several specialized crates:

- **rtp-ingest**: Receives and processes RTP audio streams
- **audio-router**: Routes audio streams between processing components
- **stt-processor**: Speech-to-text processing pipeline
- **analytics-engine**: Real-time audio analytics (speech rate, keyword detection)
- **metrics-collector**: Latency and performance metrics collection
- **websocket-api**: Real-time API for client connections
- **latency-tracker**: Distributed latency measurement across the pipeline
- **shared-types**: Common types and protocols

## Features (Planned)

- RTP stream ingestion from SIP trunks or WebRTC
- Real-time speech-to-text processing
- Keyword detection and alerting
- Speech rate analysis
- End-to-end latency tracking with metadata embedding
- Horizontal scaling with backpressure handling
- Prometheus metrics and OpenTelemetry tracing

## Development

### Prerequisites

- Rust (stable) - automatically installed via rust-toolchain.toml
- [just](https://github.com/casey/just) - command runner

### Quick Start

```bash
# Clone the repository
git clone https://github.com/navicore/navitel.git
cd navitel

# Run CI checks (format, lint, test)
just ci

# Run with tracing
just run-with-tracing

# Watch for changes and run checks
just watch
```

### Available Commands

```bash
just          # List all available commands
just fmt      # Format code
just lint     # Run clippy checks
just test     # Run tests
just build    # Build release binaries
just check    # Quick compile check
```

## CI/CD

This project uses GitHub Actions for CI/CD with identical checks running locally and in CI via the `just ci` command. Clippy and rustfmt settings are enforced through `.cargo/config.toml` to ensure consistency across all environments.

## License

MIT License - See LICENSE file for details

## Status

🚧 **Early Development** - This is a learning project exploring telephony and real-time audio processing concepts.
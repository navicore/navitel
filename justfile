# Justfile - these commands run IDENTICALLY locally and in CI
# Run 'just' to see all commands

default:
    @just --list

# Run all checks that CI will run
ci: lint test

# Format code
fmt:
    cargo fmt --all

# Run clippy with EXACT same settings as CI (via .cargo/config.toml)
lint:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features
    cargo check --all-targets --all-features

# Run tests
test:
    cargo test --all-features --workspace

# Build release
build:
    cargo build --release --all-features

# Clean build artifacts
clean:
    cargo clean

# Watch for changes and run checks
watch:
    cargo watch -x check -x test -x clippy

# Quick check - faster than full CI
check:
    cargo check --all-targets --all-features

# Run with instrumentation
run-with-tracing:
    RUST_LOG=debug,navitel=trace cargo run

# Update dependencies
update:
    cargo update
    cargo outdated
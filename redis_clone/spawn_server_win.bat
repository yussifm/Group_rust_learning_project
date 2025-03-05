@echo off

REM This script builds and runs your Rust project (in release mode),
REM forwarding any arguments you provide.

set CARGO_MANIFEST_PATH=%~dp0Cargo.toml
cargo run --quiet --release --manifest-path %CARGO_MANIFEST_PATH% -- %*

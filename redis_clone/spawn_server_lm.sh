#!/usr/bin/env bash

# This script builds and runs your Rust project (in release mode),
# forwarding any arguments you provide.

exec cargo run \
  --quiet \
  --release \
  --manifest-path "$(dirname "$0")/Cargo.toml" \
  -- "$@"

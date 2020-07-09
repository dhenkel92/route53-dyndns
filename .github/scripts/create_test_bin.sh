#! /bin/bash

set -ex

# Install rust nightly
rm -Rf ./target
rustup install nightly
rustup default nightly

# Compile test
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Clink-dead-code"
cargo test --no-run

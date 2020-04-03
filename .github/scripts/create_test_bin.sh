#! /bin/bash

set -ex

# Install rust nightly
rm -Rf ./target
rustup install nightly
rustup default nightly

# Compile test
export CARGO_INCREMENTAL=1
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads --cfg procmacro2_semver_exempt"
cargo test --no-run

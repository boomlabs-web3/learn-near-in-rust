#!/usr/bin/env bash

set -e && RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release && mkdir -p ../export && cp target/wasm32-unknown-unknown/release/*.wasm ../export/token.wasm

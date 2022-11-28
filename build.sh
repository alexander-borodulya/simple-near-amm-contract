#!/bin/bash
set -e
cd "`dirname $0`"
mkdir -p res
rustup target add wasm32-unknown-unknown
RUSTFLAGS='-C link-arg=-s' cargo build --all --target wasm32-unknown-unknown --release
cp -fv target/wasm32-unknown-unknown/release/*.wasm ./res/

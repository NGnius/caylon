#!/bin/bash

echo " --- Rust version info ---"
rustup --version
rustc --version
cargo --version

echo " --- Building plugin backend ---"
cargo build --release --features decky
mkdir -p out
cp target/release/caylon out/backend

echo " --- Cleaning up backend ---"
# remove root-owned target folder
cargo clean

echo " --- Building plugin frontend WASM ---"
# TODO allow Dockerfile in root so that it can access src/usdpl_front and rebuild it
cd ../src/usdpl_front && ./rebuild.sh decky && cd ../../backend

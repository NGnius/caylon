#!/bin/bash

export USDPL_ENCRYPTION_KEY=$(openssl enc -aes-256-cbc -k caylon -pbkdf2 -P -md sha1 | awk -F= '{if ($1 == "key") print $2}')

echo " --- Rust version info ---"
rustup --version
rustc --version
cargo --version

echo " --- Building plugin backend ---"
cargo build --release --features encrypt
mkdir -p out
cp target/release/caylon out/backend

echo " --- Cleaning up backend ---"
# remove root-owned target folder
cargo clean

echo " --- Building plugin frontend WASM ---"
# TODO allow Dockerfile in root so that it can access src/usdpl_front and rebuild it
cd ../src/usdpl_front && ./rebuild.sh decky encrypt && cd ../../backend

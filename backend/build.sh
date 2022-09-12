#!/bin/bash

#cargo build --release --target x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl
#cross build --release

mkdir -p ../bin
#cp ./target/x86_64-unknown-linux-musl/release/kaylon ../bin/backend
cp ./target/x86_64-unknown-linux-musl/debug/kaylon ../bin/backend
#cp ./target/release/kaylon ../bin/backend

cp ../kaylon.json ../bin/kaylon.json

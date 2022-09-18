#!/bin/bash

#cargo build --release --target x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl
#cross build --release

mkdir -p ../bin
#cp ./target/x86_64-unknown-linux-musl/release/caylon ../bin/backend
cp ./target/x86_64-unknown-linux-musl/debug/caylon ../bin/backend
#cp ./target/release/caylon ../bin/backend

cp ../caylon.json ../bin/caylon.json

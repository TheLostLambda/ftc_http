#!/bin/sh

rm Cargo.lock
cargo clean
cargo build --release
cargo build --release --target x86_64-pc-windows-gnu
export LD_LIBRARY_PATH=/opt/osxcross/lib/
cargo build --release --target x86_64-apple-darwin

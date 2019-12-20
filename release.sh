#!/bin/sh

rm Cargo.lock
cargo clean
cargo build --release
cargo build --release --target x86_64-pc-windows-gnu
export LD_LIBRARY_PATH=/opt/osxcross/lib/
cargo build --release --target x86_64-apple-darwin

mkdir target/bin
cp target/release/ftc_http target/bin/ftc_http_lin
cp target/x86_64-apple-darwin/release/ftc_http target/bin/ftc_http_mac
cp target/x86_64-pc-windows-gnu/release/ftc_http.exe target/bin/ftc_http_win.exe

cd target/bin/
zip ftc_http.zip *
mv ftc_http.zip ../

#!/bin/sh

rustup update
rm Cargo.lock
cargo clean
cargo build --release
export CHANNEL=nightly
for lib in crt2.o dllcrt2.o libmsvcrt.a;
do cp -v /usr/x86_64-w64-mingw32/lib/$lib $HOME/.rustup/toolchains/$CHANNEL-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/;
done
cargo build --release --target x86_64-pc-windows-gnu
export LD_LIBRARY_PATH=/opt/osxcross/lib/
cargo build --release --target x86_64-apple-darwin

mkdir target/bin
cp target/release/ftc_http target/bin/ftc_http_lin
cp target/x86_64-apple-darwin/release/ftc_http target/bin/ftc_http_mac
cp target/x86_64-pc-windows-gnu/release/ftc_http.exe target/bin/ftc_http_win.exe

strip target/bin/ftc_http_lin

cd target/bin/
zip ftc_http.zip *
mv ftc_http.zip ../

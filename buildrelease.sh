#!/bin/sh

rm ./release/*

i="x86_64-unknown-linux-gnu"
cargo build -r --target "$i" && tar -C target/$i/release -czvf ./release/ddathome-rust-linux-x86_64.tar.gz ddathome-rust

i="x86_64-pc-windows-gnu"
cargo build -r --target "$i" && tar -C target/$i/release -czvf ./release/ddathome-rust-windows-x86_64.tar.gz ddathome-rust.exe
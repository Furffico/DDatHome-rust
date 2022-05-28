#!/bin/sh

mkdir dummy

echo 'fn main(){}' > ./dummy/dummy.rs
echo '''
[package]
name = "dummy"
version = "0.0.1"
edition = "2021"
[[bin]]
name = "dummy"
path = "dummy.rs"

''' > ./dummy/Cargo.toml
grep -A 100 '^\[dependencies\]' ./Cargo.toml >> ./dummy/Cargo.toml
cp ./Cargo.lock ./dummy/

version=$(grep -E '^version *= *"[0-9]+\.[0-9]+\.[0-9]+"' ./Cargo.toml | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+')

docker build . -t "ddathome-rust:$version"
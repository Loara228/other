#!/bin/bash

sudo apt-get install gcc-mingw-w64-x86-64 -y
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu

#!/usr/bin/env sh
RUSTFLAGS='-C link-arg=-s' cargo build --release --target "$1" && upx --best --lzma "./target/$1/release/discac"
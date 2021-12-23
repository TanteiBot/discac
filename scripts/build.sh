#!/usr/bin/env sh
RUSTFLAGS="-C link-arg=-s" /usr/bin/env sh -c cargo build --release --target "$1"

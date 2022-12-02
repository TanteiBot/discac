#!/usr/bin/env sh
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.65-buster ./scripts/publish-x86-64-Linux.sh

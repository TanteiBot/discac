#!/usr/bin/env sh
./build.sh "$1" && mkdir -p "./output/" && cp "./target/$1/release/discac" "./output/discac" && cp temp.config.json "./output/temp.config.json"
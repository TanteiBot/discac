#!/usr/bin/env sh
/usr/bin/env sh ./build.sh "$1" && mkdir -p "./output/$1" && cp "./target/$1/release/discac" "./output/$1/discac" && cp temp.config.json "./output/$1/temp.config.json"
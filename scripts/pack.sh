#!/usr/bin/env sh
./scripts/publish-x86-64-Linux.sh && cp -r systemd/ output/ && cp LICENSE output/ && zip -9 -r x86_64-linux.zip output/

#!/bin/bash

ARCH="$1"

if [[ -z "$1" ]]; then
    echo "Usage: $0 <target arch>"
    exit 1
fi

curl -fsSL https://bun.sh/install | bash
export PATH="$HOME/.bun/bin:$PATH"

bunx --bun concurrently \
    -p "{name}:" \
    --pad-prefix \
    -n "Alpine,Arch,Debian,RPM" \
    -c "white.bold,blue.bold,yellow.bold,red.bold" \
    "cd apk && bash ./run-alpine.sh $ARCH" \
    "cd aur && bash ./build.sh" \
    "cd deb && bash ./build.sh $ARCH" \
    "cd rpm && bash ./build.sh $ARCH"

#!/bin/bash

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

if [[ "$OS" = "darwin" ]]; then
    OS="macos"
fi

if [[ "$ARCH" = "x86_64" ]]; then
    ARCH="amd64"
fi

TMPDIR="$(mktemp -d)"
FILE="$TMPDIR/pkl"
CONF="config.pkl"

if [[ ! -z "$1" ]]; then
    CONF="$1"
fi

curl -fsSLo "$FILE" "https://github.com/apple/pkl/releases/download/0.27.0/pkl-$OS-$ARCH"
chmod a+rx "$FILE"

"$FILE" eval -f toml "$CONF" -o ModHost.toml
rm -rf "$TMPDIR"

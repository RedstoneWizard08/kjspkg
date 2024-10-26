#!/bin/bash

curl -fsSL https://bun.sh/install | bash
[[ ! -d "/usr/local/bin" ]] && mkdir -p "/usr/local/bin"
cp "$HOME/.bun/bin/bun" /usr/local/bin/bun
cp "$HOME/.bun/bin/bunx" /usr/local/bin/bunx

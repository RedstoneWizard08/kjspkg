#!/bin/bash

curl -fsSL https://bun.sh/install | bash
sudo cp "$HOME/.bun/bin/bun" /usr/local/bin/bun
sudo cp "$HOME/.bun/bin/bunx" /usr/local/bin/bunx

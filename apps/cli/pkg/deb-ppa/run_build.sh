#!/bin/bash

#################################################
# This script is NOT intended for human/CI use. #
#################################################

set -euxo pipefail

TARGET="$DEB_TARGET_GNU_CPU-unknown-linux-$DEB_TARGET_ARCH_LIBC"
DIR="$(dirname "$(realpath "$0")")"

export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$HOME/.cargo/bin:$PATH"

curl -fsSL https://sh.rustup.rs | sh -s - -q -y --no-modify-path
curl -fsSL https://github.com/cargo-bins/cargo-binstall/raw/main/install-from-binstall-release.sh | bash
cargo binstall cargo-zigbuild -y --force
curl -fsSL https://raw.githubusercontent.com/tristanisham/zvm/master/install.sh | bash
export PATH="$HOME/.zvm/self:$HOME/.zvm/bin:$PATH"
zvm install 0.13.0 --force

cd "$DIR/source"
rustup target add "$TARGET"
cargo zigbuild --target "$TARGET" --release --bin kjspkg
cd ..
cp "$DIR/source/target/$TARGET/release/kjspkg" ./kjspkg

#!/bin/bash

SOURCE="https://github.com/RedstoneWizard08/kjspkg"
DIR="$(dirname "$(realpath "$0")")"

if [[ -z "$1" ]]; then
    echo "Usage: $0 <arch>"
    echo
    echo " - arch: The architecture to build for."
    echo "         x86_64/i686/aarch64/armv7h/armv7l/etc." 
    exit 1
fi

TARGET_ARCH="$1"
RUNTIME="gnu"
DEB_ARCH="$TARGET_ARCH"

if [[ "$TARGET_ARCH" = "armv7"* ]]; then
    TARGET_ARCH="arm"
    RUNTIME="gnueabihf"
    DEB_ARCH="armhf"
elif [[ "$TARGET_ARCH" = "armhf" ]]; then
    TARGET_ARCH="arm"
    RUNTIME="gnueabihf"
elif [[ "$TARGET_ARCH" = "aarch64" ]]; then
    DEB_ARCH="arm64"
elif [[ "$TARGET_ARCH" = "arm64" ]]; then
    TARGET_ARCH="aarch64"
elif [[ "$TARGET_ARCH" = "x86_64" ]]; then
    DEB_ARCH="amd64"
elif [[ "$TARGET_ARCH" = "amd64" ]]; then
    TARGET_ARCH="x86_64"
elif [[ "$TARGET_ARCH" = "i686" ]]; then
    DEB_ARCH="i386"
elif [[ "$TARGET_ARCH" = "i386" ]]; then
    TARGET_ARCH="i686"
else
    echo "Unknown architecture: $TARGET_ARCH"
    exit 1
fi

TARGET="$TARGET_ARCH-unknown-linux-$RUNTIME"

echo "Building:"
echo
echo " - Rust: $TARGET"
echo " - Debian: $DEB_ARCH"
echo

if [[ ! -d ".source" ]]; then
    git clone --depth 1 -q -b main "$SOURCE" .source
fi

LOG_FILE="$DIR/build.log"
[[ -f "$LOG_FILE" ]] && rm -f "$LOG_FILE"

echo ">> Creating package files..."
[[ -d "pkg" ]] && rm -rf pkg
mkdir pkg
[[ ! -d "pkg/DEBIAN" ]] && mkdir -p pkg/DEBIAN
cp control.in pkg/DEBIAN/control
sed -i "s/Version: git/Version: $PKG_VERSION/g" pkg/DEBIAN/control
sed -i "s/^Architecture:/Architecture: $DEB_ARCH/g" pkg/DEBIAN/control

echo ">> Installing tools..."
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$HOME/.cargo/bin:$PATH"

curl -fsSL https://sh.rustup.rs | sh -s - -q -y --no-modify-path >> "$LOG_FILE" 2>&1
curl -fsSL https://github.com/cargo-bins/cargo-binstall/raw/main/install-from-binstall-release.sh | bash >> "$LOG_FILE" 2>&1
cargo binstall cargo-zigbuild -y --force >> "$LOG_FILE" 2>&1
curl -fsSL https://raw.githubusercontent.com/tristanisham/zvm/master/install.sh | bash >> "$LOG_FILE" 2>&1
export PATH="$HOME/.zvm/self:$HOME/.zvm/bin:$PATH"
zvm install 0.13.0 --force >> "$LOG_FILE" 2>&1

cd .source
_CARGO_VER="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "kjspkg").version')"
_GIT_VER="$(git rev-parse --short HEAD)"
PKG_VERSION="$_CARGO_VER.git+$_GIT_VER"
cd "$DIR"

cd .source
echo ">> Installing Rust target..."
rustup target add "$TARGET" >> "$LOG_FILE" 2>&1
echo ">> Starting build..."
cargo zigbuild --release --target "$TARGET" --bin kjspkg
cd "$DIR"
[[ ! -d "pkg/usr/bin" ]] && mkdir -p "pkg/usr/bin"
cp -f ".source/target/$TARGET/release/kjspkg" "pkg/usr/bin/kjspkg"

echo ">> Creating package..."
DEB_NAME="kjspkg-cli_$PKG_VERSION-1_$DEB_ARCH.deb"
dpkg-deb --build pkg "$DEB_NAME" >> "$LOG_FILE" 2>&1
rm -rf pkg
echo ">> Done: $DEB_NAME"

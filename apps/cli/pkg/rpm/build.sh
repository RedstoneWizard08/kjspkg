#!/bin/bash

set -euo pipefail

OS="$(cat /etc/os-release | grep '^ID=' | cut -d '=' -f 2)"

_CARGO_VER="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "kjspkg").version')"
_GIT_VER="$(git rev-parse --short HEAD)"
PKG_VERSION="$_CARGO_VER.git+$_GIT_VER"
TAR_NAME="kjspkg-cli-$PKG_VERSION.tar.gz"

if [[ ! -d SOURCES ]] || [[ ! -f "SOURCES/$TAR_NAME" ]]; then
    mkdir -p SOURCES
    wget -qO "SOURCES/$TAR_NAME.tmp" "https://github.com/RedstoneWizard08/kjspkg/archive/refs/heads/main.tar.gz" --show-progress
    mkdir -p SOURCES/.tmp
    tar -C SOURCES/.tmp -zxvf "SOURCES/$TAR_NAME.tmp"
    rm -f "SOURCES/$TAR_NAME.tmp"
    mv SOURCES/.tmp/kjspkg-main SOURCES/.tmp/kjspkg-git
    cd SOURCES/.tmp
    tar -czvf "../$TAR_NAME" kjspkg-git
    cd ../..
    rm -rf SOURCES/.tmp
fi

DEB_VAL=0

if [[ "$OS" = "ubuntu" ]] || [[ "$OS" = "debian" ]]; then
    DEB_VAL=1
fi

cp kjspkg-cli.spec.in kjspkg-cli.spec
sed -i "s/Version: git/Version: $PKG_VERSION/g" kjspkg-cli.spec

rpmbuild \
    -ba \
    --build-in-place \
    --define "_topdir $(pwd)" \
    --define "__debian $DEB_VAL" \
    kjspkg-cli.spec

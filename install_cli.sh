#!/bin/bash

set -euxo pipefail

KJSPKG_VERSION="${KJSPKG_VERSION:-latest}"

cd "$(mktemp -d)"

base_url="https://github.com/RedstoneWizard08/kjspkg/releases/${KJSPKG_VERSION}/download/kjspkg-"

os="$(uname -s)"
if [ "$os" == "Darwin" ]; then
    url="${base_url}universal-apple-darwin.zip"
    curl -A "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/81.0" -LO --proto '=https' --tlsv1.2 -sSf "$url"
    unzip kjspkg-universal-apple-darwin.zip
elif [ "$os" == "Linux" ]; then
    machine="$(uname -m)"
    if [ "$machine" == "armv7l" ]; then
        machine="arm"
    fi
    target="${machine}-unknown-linux-musl"
    if [ "$machine" == "armv7" ]; then
        target="${target}eabihf"
    fi

    url="${base_url}${target}.tgz"
    curl -A "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/81.0" -L --proto '=https' --tlsv1.2 -sSf "$url" | tar -xvzf -
elif [ "${OS-}" = "Windows_NT" ]; then
    machine="$(uname -m)"
    target="${machine}-pc-windows-msvc"
    url="${base_url}${target}.zip"
    curl -A "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/81.0" -LO --proto '=https' --tlsv1.2 -sSf "$url"
    unzip "kjspkg-${target}.zip"
else
    echo "Unsupported OS ${os}"
    exit 1
fi

LOCAL_BINS="${LOCAL_BINS:-$HOME/.local/bin}"

if ! [[ ":$PATH:" == *":$LOCAL_BIN:"* ]]; then
    if [ -n "${CI:-}" ] && [ -n "${GITHUB_PATH:-}" ]; then
        echo "$LOCAL_BINS" >> "$GITHUB_PATH"
    else
        echo
        printf "\033[0;31mYour path is missing %s, you might want to add it.\033[0m\n" "$LOCAL_BINS"
        echo
    fi
fi

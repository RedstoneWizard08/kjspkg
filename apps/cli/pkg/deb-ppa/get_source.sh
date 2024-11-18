#!/bin/bash

#################################################
# This script is NOT intended for human/CI use. #
#################################################

set -euxo pipefail

SOURCE="https://github.com/RedstoneWizard08/kjspkg"
DIR="$(dirname "$(realpath "$0")")"

if [[ ! -d "$DIR/source" ]]; then
    git clone -q -b main "$SOURCE" "$DIR/source"
    cd "$DIR/source"
    git checkout "$(cat "$DIR/commit.txt")"
    cd ..
fi

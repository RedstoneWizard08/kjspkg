#!/bin/bash

#############################################
# This script is intended for human/CI use. #
#############################################

set -euo pipefail

DIR="$(dirname "$(realpath "$0")")"

git rev-parse HEAD > "$DIR/commit.txt"

if [[ "$SOURCE" = "1" ]]; then
    debuild -S --lintian-opts --suppress-tags source-is-missing
else
    debuild -us -uc --lintian-opts --suppress-tags source-is-missing
fi

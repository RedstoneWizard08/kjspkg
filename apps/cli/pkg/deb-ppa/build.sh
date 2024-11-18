#!/bin/bash

#############################################
# This script is intended for human/CI use. #
#############################################

set -euo pipefail

DIR="$(dirname "$(realpath "$0")")"

git rev-parse HEAD > "$DIR/commit.txt"
debuild --no-lintian -us -uc

#!/bin/bash

_CARGO_VER="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "kjspkg").version')"
_GIT_VER="$(git show --no-patch --format=%cd --date=format:%Y%m%d)"

docker run --rm \
    -v "$(pwd):/pkg" \
    -e "_CARGO_VER=$_CARGO_VER" \
    -e "_GIT_VER=$_GIT_VER" \
    alpine \
    /pkg/exec-alpine.sh \
    2>&1 | tee "$(pwd)/alpine-build.log"

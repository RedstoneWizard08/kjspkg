#!/bin/ash

cd /pkg
apk add abuild alpine-sdk doas bash sudo
abuild-keygen -an
cp ~/.abuild/*.pub /etc/apk/keys

[[ -z "$CI" ]] && ./build.sh -c
[[ ! -z "$CI" ]] && ./build.sh

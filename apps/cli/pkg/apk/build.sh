#!/bin/bash

OS="$(cat /etc/os-release | grep '^ID=' | cut -d '=' -f 2)"
ABUILD="abuild"
DEB_ABUILD="/usr/local/abuild/usr/bin/abuild"

exit_debian() {
    echo "It looks like you are on Ubuntu or Debian."
    echo "You can either specify the path to 'abuild' with the first argument,"
    echo "or make sure it exists at '$DEB_ABUILD'."
    exit 1
}

if [[ "$OS" = "ubuntu" ]] || [[ "$OS" = "debian" ]]; then
    if [[ -z "$1" ]]; then
        if [[ -f "$DEB_ABUILD" ]]; then
            ABUILD="$DEB_ABUILD"
        else
            exit_debian
        fi
    else
        if command -v "$1" 2>&1 > /dev/null; then
            ABUILD="$1"
        else
            exit_debian
        fi
    fi

    CBUILD="${CBUILD:-$(uname -m)}"
    export CBUILD

    cp APKBUILD.in APKBUILD
else
    cp APKBUILD.in APKBUILD
fi

[[ -z "$_CARGO_VER" ]] && _CARGO_VER="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "kjspkg").version')"
[[ -z "$_GIT_VER" ]] && _GIT_VER="$(git show --no-patch --format=%cd --date=format:%Y%m%d)"

PKG_VERSION="${_CARGO_VER}_git$_GIT_VER"
sed -i "s/pkgver=git/pkgver=$PKG_VERSION/g" APKBUILD

"$ABUILD" -FrK "$@"
[[ -d "output" ]] && rm -rf output
cp -r ~/packages ./output

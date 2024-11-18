#!/bin/bash

OS="$(cat /etc/os-release | grep '^ID=' | cut -d '=' -f 2)"
MAKEPKG="makepkg"
DEB_MAKEPKG="/usr/share/pacman/bin/makepkg"

exit_debian() {
    echo "It looks like you are on Ubuntu or Debian."
    echo "You can either specify the path to 'makepkg' with the first argument,"
    echo "or make sure it exists at '$DEB_MAKEPKG'."
    exit 1
}

if [[ "$OS" = "ubuntu" ]] || [[ "$OS" = "debian" ]]; then
    if [[ -z "$1" ]]; then
        if [[ -f "$DEB_MAKEPKG" ]]; then
            MAKEPKG="$DEB_MAKEPKG"
        else
            exit_debian
        fi
    else
        if command -v "$1" 2>&1 > /dev/null; then
            MAKEPKG="$1"
        else
            exit_debian
        fi
    fi

    cp PKGBUILD.in PKGBUILD
    sed -i 's/makedepends=/# makedepends=/g' PKGBUILD
else
    cp PKGBUILD.in PKGBUILD
fi

if [[ "$FORCE" = "1" ]]; then
    "$MAKEPKG" -sL --force
else
    "$MAKEPKG" -sL
fi

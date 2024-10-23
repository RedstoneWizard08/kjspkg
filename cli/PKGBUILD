# Maintainer: G_cat <https://github.com/Gcat101>
pkgname=kjspkg-git
pkgver=1.0
pkgrel=1
epoch=
pkgdesc="A package manager for KubeJS."
arch=(x86_64 i686)
url="https://www.github.com/Modern-Modpacks/kjspkg.git"
license=('MIT')
groups=()
depends=(python git)
makedepends=(python-pip curl sudo)
checkdepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=("git+$url")
noextract=()
md5sums=("SKIP")
validpgpkeys=()

pkgver() {
	cd "kjspkg"
	git rev-parse --short HEAD
}

package() {
	sudo -v
	curl -s https://raw.githubusercontent.com/Modern-Modpacks/kjspkg/main/install.sh | sh
}

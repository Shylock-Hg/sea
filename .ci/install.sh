#! /usr/bin/env bash

###############################################################################
#!  \brief install stuff needed for the `script` phase
#   \author Shylock Hg
#   \date 2019-01-10
#   \email tcath2s@gmail.com
###############################################################################


# Where rustup gets installed.
export PATH="$PATH:$HOME/.cargo/bin"

readonly BASEDIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

set -ex

. "$(dirname $0)/utils.sh"

install_rustup() {
    curl https://sh.rustup.rs -sSf \
      | sh -s -- -y --default-toolchain="$(cat $BASEDIR/rust-toolchain)"
    rustc -V
    cargo -V
}

install_targets() {
    if [ $(host) != "$TARGET" ]; then
        rustup target add $TARGET
    fi
}

install_osx_dependencies() {
    if ! is_osx; then
      return
    fi

    brew install asciidoc docbook-xsl
}

configure_cargo() {
    local prefix=$(gcc_prefix)
    if [ -n "${prefix}" ]; then
        local gcc_suffix=
        if [ -n "$GCC_VERSION" ]; then
          gcc_suffix="-$GCC_VERSION"
        fi
        local gcc="${prefix}gcc${gcc_suffix}"

        # information about the cross compiler
        "${gcc}" -v

        # tell cargo which linker to use for cross compilation
        mkdir -p .cargo
        cat >>.cargo/config <<EOF
[target.$TARGET]
linker = "${gcc}"
EOF
    fi
}

main() {
#    install_osx_dependencies
    install_rustup
#    install_targets
#    configure_cargo
}

main

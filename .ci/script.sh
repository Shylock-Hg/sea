#! /usr/bin/env bash

###############################################################################
#!  \brief  
#   \author Shylock Hg
#   \date 2019-01-10
#   \email tcath2s@gmail.com
###############################################################################

# build, test and generate docs in this phase

# Where rustup gets installed.
export PATH="$PATH:$HOME/.cargo/bin"

set -ex

. "$(dirname $0)/utils.sh"

main() {
    # Test a normal debug build.
    cargo build --verbose

    # Show the output of the most recent build.rs stderr.
#    set +x
#    stderr="$(find "target/debug" -name stderr -print0 | xargs -0 ls -t | head -n1)"
#    if [ -s "$stderr" ]; then
#      echo "===== $stderr ====="
#      cat "$stderr"
#      echo "====="
#    fi
#    set -x

    # Apparently tests don't work on arm, so just bail now. I guess we provide
    # ARM releases on a best effort basis?
    if is_arm; then
      return 0
    fi

    # Run tests for ripgrep and all sub-crates.
    cargo test --verbose
}

main

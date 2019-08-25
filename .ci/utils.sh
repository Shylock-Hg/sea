###############################################################################
#!  \brief  
#         Various utility functions used through CI.
#         Finds Cargo's `OUT_DIR` directory from the most recent build.
#
#         This requires one parameter corresponding to the target directory
#         to search for the build output.
#   \author Shylock Hg
#   \date 2019-01-10
#   \email tcath2s@gmail.com
###############################################################################

cargo_out_dir() {
    # This works by finding the most recent stamp file, which is produced by
    # every ripgrep build.
    target_dir="$1"
    find "$target_dir" -name ripgrep-stamp -print0 \
      | xargs -0 ls -t \
      | head -n1 \
      | xargs dirname
}

host() {
    case $(uname) in
        Linux)
            echo x86_64-unknown-linux-gnu
            ;;
        Darwin)
            echo x86_64-apple-darwin
            ;;
    esac
}

architecture() {
    case $(uname -m) in
        x86_64-*)
            echo amd64
            ;;
        i686-*|i586-*|i386-*)
            echo i386
            ;;
        arm*-unknown-linux-gnueabihf)
            echo armhf
            ;;
        *)
            die "architecture: unexpected target $TARGET"
            ;;
    esac
}

gcc_prefix() {
    case "$(architecture)" in
        armhf)
            echo arm-linux-gnueabihf-
            ;;
        *)
            return
            ;;
    esac
}

is_x86() {
    case "$(architecture)" in
      amd64|i386) return 0 ;;
      *)          return 1 ;;
    esac
}

is_arm() {
    case "$(architecture)" in
        armhf) return 0 ;;
        *)     return 1 ;;
    esac
}

is_linux() {
    case $(uname) in
        Linux) return 0 ;;
        *)     return 1 ;;
    esac
}

is_osx() {
    case $(uname) in
        Darwin) return 0 ;;
        *)   return 1 ;;
    esac
}

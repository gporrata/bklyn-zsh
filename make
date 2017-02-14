#!/usr/bin/env zsh

# requires nodemon ($ npm global --install nodemon)

# continuously run with
# $ make watch run <cmd> <args>

# continuously build with 
# $ make watch build

# continously test with
# $ make watch test

# etc... etc...

case "$1" in
  run)
    clear
    cargo build 2>&1 >&- --color=always
    if [[ $! == "0" ]]; then
      shift
      #time (RUST_BACKTRACE=1 OSTYPE=$OSTYPE "$@" ; echo)
      time "$@" ; echo
    else
      echo "Unable to run??"
    fi
    ;;
  build)
    clear
    cargo build 2>&1 >&- --color=always --verbose | less 
    ;;
  build-release)
    clear
    cargo build --release && cp ./target/release/bklyn_zsh .
    ;;
  test)
    clear
    cargo test --color=always 2>&1 >&-| less
    ;;
  watch)
    shift
    nodemon --exec ${0:A} --ext "rs toml" -- $@ 
    ;;
esac


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
  build|run)
    clear
    cargo build 2>&1 >&- --color=always | less 
    if [[ ${pipestatus[1]} == "0" && "$1" == "run" ]]; then
      shift
      RUST_BACKTRACE=1 "$@"
    else
      echo "Unable to run??"
    fi
    ;;
  watch)
    shift
    nodemon --exec ${0:A} --ext "rs toml" -- $@ 
    ;;
esac


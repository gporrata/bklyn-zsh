#!/bin/zsh

bklyn_zsh_yaml_pad() {
  while read line; do
    echo "  $line"
  done
}

bklyn_zsh_data() {
  cat <<EOF
COLS: $1
EXIT: $2
PID: $3
PWD: $PWD
USER: $USER
HOST: $HOST
SSH_TTY: $4
SSH_CLIENT: $5
GIT: |
`git -c color.status=false status --porcelain=2 --branch | bklyn_zsh_yaml_pad`
GIT_STASH: |
`git stash list | bklyn_zsh_yaml_pad`
EOF
}

case "$1" in
  --start)
    nodemon --ignore dist --exec $0:A --test-server
    ;;
  --test-server)
    BKLYN_ZSH_PORT=65530
    rm -rf ./dist/*
    webpack --config src/webpack.config.babel.js --progress >/dev/null
    PORT=${BKLYN_ZSH_PORT} node ./dist/bklyn-zsh-bundle.js &
    server_pid=$!
    while ! nc -z localhost ${BKLYN_ZSH_PORT} 2>/dev/null; do
      sleep 0.1 # wait for 1/10 of the second before check again
    done
    docurl=(curl --data-binary @- -s -H"Content-Type:text/plain")
    clear
    echo "-- 5, then 1 timed runs of zsh-left"
    bklyn_zsh_data 1000 0 '' '' '' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-left && echo
    bklyn_zsh_data 100 0 '' '' '' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-left && echo
    bklyn_zsh_data 1000 0 '' /dev/pts/0 '88.133.195.51 57805 22' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-left && echo
    bklyn_zsh_data 100 0 '' /dev/pts/0 '88.133.195.51 57805 22' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-left && echo
    bklyn_zsh_data 00 0 '' /dev/pts/0 '88.133.195.51 57805 22' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-left && echo
    time ( bklyn_zsh_data 1000 0 '' /dev/pts/0 '88.133.195.51 57805 22' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-left && echo )
    echo "-- 5, then 1 timed runs of zsh-right"
    bklyn_zsh_data 100 0 '' '' '' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-right && echo
    bklyn_zsh_data 100 0 2344 '' '' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-right && echo
    bklyn_zsh_data 100 -1 8954 '' '' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-right && echo
    bklyn_zsh_data 100 127 7892 '' '' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-right && echo
    bklyn_zsh_data 100 -33 3049 '' '' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-right && echo
    time ( bklyn_zsh_data 100 -3 8234 '' '' | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-right && echo )
    kill ${server_pid}
    ;;
esac

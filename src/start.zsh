#!/bin/zsh

bklyn_zsh_yaml_pad() {
  while read line; do
    echo "  $line"
  done
}

bklyn_zsh_data() {
  cat <<EOF
PID: $!
EXIT: $?
PWD: $PWD
USER: $USER
HOST: $HOST
SSH_CLIENT: $SSH_CLIENT
SSH_TTY: $SSH_TTY
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
    # when using curl use "-d @-" to post data from stdin
    BKLYN_ZSH_PORT=65530
    rm -rf ./dist/*
    webpack --config src/webpack.config.babel.js --progress >/dev/null
    PORT=${BKLYN_ZSH_PORT} node ./dist/bklyn-zsh-bundle.js &
    server_pid=$!
    while ! nc -z localhost ${BKLYN_ZSH_PORT} 2>/dev/null; do
      sleep 0.1 # wait for 1/10 of the second before check again
    done
    docurl=(curl --data-binary @- -s -H"Content-Type:text/plain")
    echo -----
    bklyn_zsh_data
    echo "5 and 1 timed runs of zsh-left"
    for i in {1..5}; do
      echo -n $i\:; bklyn_zsh_data | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-left && echo
    done
    time (echo -n 6\: ; bklyn_zsh_data | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-left && echo)
    echo "5 and 1 timed runs of zsh-right"
    for i in {1..5}; do
      echo -n $i\:; bklyn_zsh_data | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-right && echo
    done
    time (echo -n 6\:; bklyn_zsh_data | $docurl http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-right && echo)
    kill ${server_pid}
    ;;
esac

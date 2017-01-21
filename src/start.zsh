#!/bin/zsh

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
    echo "5 and 1 timed runs of zsh-left"
    for i in {1..5}; do wget -qO - http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-left && echo; done
    time (wget -qO - http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-left | cat && echo)
    echo "5 and 1 timed runs of zsh-right"
    for i in {1..5}; do wget -qO - http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-right && echo; done
    time (wget -qO - http://127.0.0.1:${BKLYN_ZSH_PORT}/zsh-right | cat && echo)
    kill ${server_pid}
    ;;
esac

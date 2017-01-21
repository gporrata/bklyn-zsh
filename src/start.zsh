#!/bin/zsh

case "$1" in
  --start)
    webpack --config src/webpack.config.babel.js --watch --progress &
    nodemon --watch dist ./dist/bundle --exec $0:A --test-server
    ;;
  --test-server)
    

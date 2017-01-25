bklyn_zsh_port=9988

bklyn_zsh_existing_pid=`lsof -n -i:$bklyn_zsh_port | grep LISTEN | awk -F ' ' '{print $2}'`

kill -KILL `lsof -n -i:${bklyn_zsh_port} | grep LISTEN | awk -F ' ' '{print $2}'`
PORT=${bklyn_zsh_port} node ${0:A:h}/dist/bklyn-zsh-bundle.js &

#if ! lsof -n -i:${bklyn_zsh_port} | grep LISTEN >/dev/null; then
#  PORT=${bklyn_zsh_port} node ${0:A:h}/dist/bklyn-zsh-bundle.js &
#fi

bklyn_zsh_yaml_pad() {
  while read line; do
    echo "  $line"
  done
}

bklyn_zsh_data() {
  cat <<EOF
COLS: $COLUMNS
EXIT: $1
PID: $2
PWD: $PWD
USER: $USER
HOST: $HOST
SSH_CLIENT: $SSH_CLIENT
SSH_TTY: $SSH_TTY
GIT: |
`(git -c color.status=false status --porcelain=2 --branch 2>/dev/null || echo -n '') | bklyn_zsh_yaml_pad`
GIT_STASH: |
`(git stash list 2>/dev/null || echo -n '') | bklyn_zsh_yaml_pad`
EOF
}

bklyn_zsh_precmd_hook() {
  bklyn_zsh_last_exit=$?
  bklyn_zsh_last_pid=$!
  PROMPT=`bklyn_zsh_data $bklyn_zsh_last_exit $bklyn_zsh_last_pid | curl --data-binary @- -s -H"Content-Type:text/plain" http://127.0.0.1:${bklyn_zsh_port}/zsh-left`
  RPROMPT=`bklyn_zsh_data $bklyn_zsh_last_exit $bklyn_zsh_last_pid | curl --data-binary @- -s -H"Content-Type:text/plain" http://127.0.0.1:${bklyn_zsh_port}/zsh-right`
}

# make sure to install bklyn-zsh only once
if [[ $bklyn_zsh_installed != 'installed' ]]; then
  bklyn_zsh_installed='installed'

  [[ -z $precmd_functions ]] && precmd_functions=()
  precmd_functions=($precmd_functions bklyn_zsh_precmd_hook)
fi

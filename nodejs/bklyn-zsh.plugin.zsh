setopt NO_HUP
setopt NO_CHECK_JOBS

bklyn_zsh_dir=${0:A:h}

# default BKLYN_ZSH_SOCKET_PATH
if [[ "$BKLYN_ZSH_SOCKET_PATH" == "" ]]; then
  export BKLYN_ZSH_SOCKET_PATH=${bklyn_zsh_dir}/.bklyn_zsh_comm
fi

# if node modules missing or package.json newer, install and rebuild
bklyn_zsh_rebuild() {
  if [[ ( ! -d "${bklyn_zsh_dir}/node_modules" ) || ( "${bklyn_zsh_dir}/package.json" -nt "${bklyn_zsh_dir}/node_modules" ) ]]; then
    pushd "${bklyn_zsh_dir}"
    echo "Rebuilding in `pwd`"
    yarn
    yarn run build
    popd
    bklyn_zsh_rebuild="rebuild"
  else
    bklyn_zsh_rebuild=
  fi
}

bklyn_zsh_restart_server() {
  # always restart server in debug mode or if server rebuild
  if [[ ( "$bklyn_zsh_debug" == "debug" ) || ( "$bklyn_zsh_rebuild" == "rebuild" ) ]]; then
    rm -f "${BKLYN_ZSH_SOCKET_PATH}"
    node ${bklyn_zsh_dir}/dist/bklyn-zsh-bundle.js &|
  # otherwise its ok to not restart server
  elif [[ ! -e "${BKLYN_ZSH_SOCKET_PATH}" ]] || (( `ps ax | grep -c "${bklyn_zsh_dir}/dist/bklyn-zsh-bundle.js"` <= 1 )); then
    rm -f "${BKLYN_ZSH_SOCKET_PATH}"
    NODE_ENV=production node ${bklyn_zsh_dir}/dist/bklyn-zsh-bundle.js &|
  fi
  # wait for server to start
  while ! echo '' | nc -vU "${BKLYN_ZSH_SOCKET_PATH}"; do
    sleep 0.1
  done
}

# build and restart
bklyn_zsh_rebuild_and_restart() {
  bklyn_zsh_rebuild
  bklyn_zsh_restart_server
}

# for padding yaml 'here doc' data
bklyn_zsh_yaml_pad() {
  while read line; do
    echo "  $line"
  done
}

# passes data to server
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

# called prior to every command
bklyn_zsh_precmd_hook() {
  bklyn_zsh_rebuild_and_restart

  # override to just get prompt data
  bklyn_zsh_precmd_hook() {
    bklyn_zsh_last_pid=$!
    bklyn_zsh_last_exit=$?
    PROMPT=`bklyn_zsh_data $bklyn_zsh_last_exit $bklyn_zsh_last_pid | curl --data-binary @- -s -H"Content-Type:text/plain" --unix-socket ${BKLYN_ZSH_SOCKET_PATH} http://localhost/zsh-left`
    RPROMPT=`bklyn_zsh_data $bklyn_zsh_last_exit $bklyn_zsh_last_pid | curl --data-binary @- -s -H"Content-Type:text/plain" --unix-socket ${BKLYN_ZSH_SOCKET_PATH} http://localhost/zsh-right`
  }

  bklyn_zsh_precmd_hook
}

# install bklyn_zsh
bklyn_zsh_install() {
  # make sure to install bklyn-zsh only once
  if [[ $bklyn_zsh_installed != 'installed' ]]; then
    bklyn_zsh_installed='installed'

    [[ -z $precmd_functions ]] && precmd_functions=()
    precmd_functions=($precmd_functions bklyn_zsh_precmd_hook)
  fi
}

bklyn_zsh_install

setopt NO_HUP
setopt NO_CHECK_JOBS

bklyn_zsh_dir=${0:A:h}

# builds bklyn_zsh
bklyn_zsh_build() {
  path_now=$PATH
  if ! which cargo >/dev/null; then
    if [[ -f ~/.cargo/bin/cargo ]]; then
      export PATH="$PATH:$HOME/.cargo/bin"
    else
      echo "Failed to find rust+cargo"
    fi
  fi
  ( 
    cd "$bklyn_zsh_dir" &&
    cargo build --release &&
    cp "$bklyn_zsh_dir/target/release/bklyn_zsh" "$bklyn_zsh_dir" &&
    rm -rf target
  ) || 
  echo "Failed to build bklyn_zsh"
  export PATH=$path_now
}

# called prior to every command
bklyn_zsh_precmd_hook() {
  # build if missing bklyn_zsh
  if [[ ! -f "$bklyn_zsh_dir/bklyn_zsh" ]]; then
    bklyn_zsh_build
  fi

  # modify hook to stop checking for bklyn_zsh bin
  bklyn_zsh_precmd_hook() {
    bklyn_zsh_exit_code=$? bklyn_zsh_pid=$$
    PROMPT=`OSTYPE=$OSTYPE HOST=$HOST $bklyn_zsh_dir/bklyn_zsh -p zsh-left ssh os dir git`
    RPROMPT=`bklyn_zsh_pid=$bklyn_zsh_pid bklyn_zsh_exit_code=$bklyn_zsh_exit_code $bklyn_zsh_dir/bklyn_zsh -p zsh-right exit pid`
  }

  bklyn_zsh_precmd_hook
}

# install bklyn_zsh hook
bklyn_zsh_install() {
  [[ -z $precmd_functions ]] && precmd_functions=()
  precmd_functions=($precmd_functions bklyn_zsh_precmd_hook)
}

# install bklyn-zsh hook only once
if [[ $bklyn_zsh_installed != 'installed' ]]; then
  bklyn_zsh_install
  bklyn_zsh_installed='installed'
fi


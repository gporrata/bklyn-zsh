setopt NO_HUP
setopt NO_CHECK_JOBS

bklyn_zsh_dir=${0:A:h}

bklyn_zsh_setup_rust() {
  if which carg >/dev/null; then
    return 0
  elif [[ -f ~/.cargo/bin/cargo ]]; then
    export PATH="$PATH:$HOME/.cargo/bin"
    return 0
  else
    echo "Failed to find rust+cargo"
    return -1
  fi
}

# builds bklyn_zsh
bklyn_zsh_build() {
  ( 
    cd "$bklyn_zsh_dir" &&
    bklyn_zsh_setup_rust &&
    cargo build --release &&
    mkdir -p "$bklyn_zsh_dir/bin" &&
    cp "$bklyn_zsh_dir/target/release/bklyn_zsh" "$bklyn_zsh_dir/bin"
  ) || 
  echo "Failed to build bklyn_zsh"
}

# called prior to every command
bklyn_zsh_precmd_hook() {
  # build if missing bklyn_zsh
  if [[ ! -f "$bklyn_zsh_dir/bin/bklyn_zsh" ]]; then
    bklyn_zsh_build
  fi

  # install if build
  if [[ -f "$bklyn_zsh_dir/bin/bklyn_zsh" ]]; then
    export PATH="$PATH:$bklyn_zsh_dir/bin" 
    
    # modify hook to stop checking for bklyn_zsh bin
    bklyn_zsh_precmd_hook() {
      bklyn_zsh_exit_code=$? bklyn_zsh_pid=$$
      PROMPT=`bklyn_zsh -p zsh-left ssh os dir git`
      RPROMPT=`bklyn_zsh_pid=$bklyn_zsh_pid bklyn_zsh_exit_code=$bklyn_zsh_exit_code bklyn_zsh -p zsh-right exit pid`
    }

    bklyn_zsh_precmd_hook
  fi
}

# install bklyn_zsh hook
bklyn_zsh_install() {
  export OSTYPE
  export host
  [[ -z $precmd_functions ]] && precmd_functions=()
  precmd_functions=($precmd_functions bklyn_zsh_precmd_hook)
}

# install bklyn-zsh hook only once
if [[ $bklyn_zsh_installed != 'installed' ]]; then
  bklyn_zsh_install
  bklyn_zsh_installed='installed'
fi


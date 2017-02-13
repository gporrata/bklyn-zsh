setopt NO_HUP
setopt NO_CHECK_JOBS

bklyn_zsh_dir=${0:A:h}

# called prior to every command
bklyn_zsh_precmd_hook() {
  # if bklyn_zsh missing, build it
  if [[ ! -f "${bklyn_zsh_dir}/bklyn_zsh" ]]; then
    cargo build --release
    cp ./target/release/bklyn_zsh .
    rm -rf target
  fi

  # reinstall bklyn_zsh_precmd_hook
  bklyn_zsh_precmd_hook() {
    bklyn_zsh_exit_code=$? bklyn_zsh_pid=$$
    PROMPT=`OSTYPE=$OSTYPE ${bklyn_zsh_dir}/bklyn_zsh -p zsh-left os dir git`
    RPROMPT=`bklyn_zsh_pid=$bklyn_zsh_pid bklyn_zsh_exit_code=$bklyn_zsh_exit_code ${bklyn_zsh_dir}/bklyn_zsh -p zsh-right exit pid`
  }

  bklyn_zsh_precmd_hook()
}

# install bklyn_zsh hook
bklyn_zsh_install() {
  # make sure to install bklyn-zsh only once
  if [[ $bklyn_zsh_installed != 'installed' ]]; then
    bklyn_zsh_installed='installed'

    [[ -z $precmd_functions ]] && precmd_functions=()
    precmd_functions=($precmd_functions bklyn_zsh_precmd_hook)
  fi
}

bklyn_zsh_install

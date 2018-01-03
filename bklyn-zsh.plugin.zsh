setopt NO_HUP
setopt NO_CHECK_JOBS

bklyn_zsh_dir=${0:A:h}

export PATH="$bklyn_zsh_dir:$PATH"

bklyn_zsh_update() {
  for f in color_codes ostype ssh git prompt runtime install; do
    source ${bklyn_zsh_dir}/src/$f.zsh
  done
}

bklyn_zsh_update

up() {
  bklyn_zsh_update
}

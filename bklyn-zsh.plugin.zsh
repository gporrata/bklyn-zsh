setopt NO_HUP
setopt NO_CHECK_JOBS

bklyn_zsh_dir=${0:A:h}

up() {
  for f in color_codes install ostype git prompt; do
    source ${bklyn_zsh_dir}/src/$f.zsh
  done
}

#up

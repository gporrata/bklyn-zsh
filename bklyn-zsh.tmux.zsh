#!/usr/bin/env zsh

bklyn_zsh_dir=${0:A:h}

for f in ostype; do
  source $bklyn_zsh_dir/src/$f.zsh
done

left_sep=$'\ue0b4'
left_sep_thin=$'\ue0b5'

right_color='#[bg=#222222]'
right_sep='#[fg=#222222]'$'\ue0b6'
right_sep_thin=$'\ue0b7'
load_icon=$'\ue234'
load_color='#[fg=#4db6ac]'
uptime_color='#[fg=#90caf9]'
time_color='#[fg=#ffeb3b]'
time_icon=$'\uf43a'

case "$1" in
  left)
    echo " #[fg=$bklyn_zsh_ostype_color_code]$bklyn_zsh_ostype "
    ;;
  right)
    right_repl="$right_sep$right_color$load_color$load_icon \4 \5 \6 $time_color$time_icon \1"
    right_line=`uptime | sed -nE "s/([^ ]+)[ ]+(([^,]+)[ ]+)?[0-9:]+,[ ]+[^,]+,[ ]+load average: ([^, ]+), ([^, ]+), ([^, ]+)/$right_repl/ p"`
    echo $right_line
    ;;
esac

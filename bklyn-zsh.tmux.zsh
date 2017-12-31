#!/usr/bin/env zsh

bklyn_zsh_dir=${0:A:h}

for f in color_codes ostype; do
  source $bklyn_zsh_dir/src/$f.zsh
done

fg() {
  echo -n "#[fg=$1]"
}

bg() {
  echo -n "#[bg=$1]"
}

fbg() {
  echo -n "#[fg=$1]#[bg=$2]"
}

tmux_color='#0f3247'

left_sep=$'\ue0b4'
left_sep_thin=$'\ue0b5'

center_color='#1a237e'
active_center_color='#e1f5fe'
active_icon=$'\uf067'
inactive_icon=$'\uf068'

right_color='#e1f5fe'
right_sep=$'\ue0b6'
right_sep_thin=$'\ue0b7'
load_icon=$'\ue234'
load_color='#004d40,bold'
time_color='#b71c1c,bold'
time_icon=$'\uf43a'

center_debug() {
  echo "$center_color$@"
}

eval_center() {
  # $1 : index of window
  # $2 : count of windows
  # $3 : 1 if active
  # $4 : title of window
  title="$1 ${4%%:*}"
  if [[ $3 == 1 ]]; then
    echo -n "$(fg $active_center_color)$right_sep$(fbg "$center_color,bold" $active_center_color)$active_icon $title$(fbg $active_center_color $tmux_color)$left_sep "
  elif [[ $1 == 0 ]]; then
    echo -n "$(fg $active_center_color)$right_sep_thin $inactive_icon $title $left_sep_thin "
  else
    echo -n "$(fg $active_center_color)$inactive_icon $title $left_sep_thin "
  fi
}

case "$1" in
  left)
    echo " $(fg $active_center_color)$bklyn_zsh_ostype "
    ;;
  center)
    shift
    #center_debug $@
    eval_center $@ 
    ;;
  right)
    right_repl="$(fg $right_color)$right_sep$(bg $right_color)$(fg $load_color)$load_icon \4 \5 \6 $(fg $time_color)$time_icon \1 "
    right_line=`uptime | sed -nE "s/([^ ]+)[ ]+(([^,]+)[ ]+)?[0-9:]+,[ ]+[^,]+,[ ]+load average: ([^, ]+), ([^, ]+), ([^, ]+)/$right_repl/ p"`
    echo $right_line
    ;;
  *)
    echo "Unknown specifier $1"
    ;;
esac

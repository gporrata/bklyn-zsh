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

center_color='#43a047'
active_center_color='#111111'
inactive_center_color='#dddddd'
active_icon=$'\uf055'
inactive_icon=$'\uf056'

right_color='#111111'
right_sep=$'\ue0b6'
right_sep_thin=$'\ue0b7'

disk_icon=$'\uf233'
disk_color='#a1887f'

cpu_icon=$'\ue266'
cpu_color='#42a5f5'

load_icon=$'\ue234'
load_color='#43a047,bold'

time_color='#e65100,bold'
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
    echo -n "$(fg $inactive_center_color)$right_sep_thin $inactive_icon $title $left_sep_thin "
  else
    echo -n "$(fg $inactive_center_color)$inactive_icon $title $left_sep_thin "
  fi
}

linux_right() {
}

osx_right() {
  spc=" +"
  val="([.0-9]+)"
  i=$spc$val
  sub=$i$i$i$i$i$i$i$i$i
  repl="$(fg $disk_color)$right_sep_thin $disk_icon \1 \2 \3 $(fg $cpu_color)$right_sep_thin $cpu_icon \4 \5 \6 $(fg $load_color)$right_sep_thin $load_icon \7 \8 \9 "
  stats=$(iostat | tail -n1 | sed -nE "s/$sub$/$repl/ p")
  time_info="$(fg $time_color)$right_sep_thin $time_icon $(date '+%H:%M')"
  right_line="$stats $time_info" 
  echo $right_line
}

uptime_right() {
  right_repl="$(fg $right_color)$right_sep$(bg $right_color)$(fg $load_color)$load_icon \4 \5 \6 $right_sep_thin $(fg $time_color)$time_icon \1 "
  right_line=`uptime | sed -nE "s/([^ apm]+)[apm]+[ ]+(([^,]+)[ ]+)?[0-9:]+,[ ]+[^,]+,[ ]+load average: ([^, ]+), ([^, ]+), ([^, ]+)/$right_repl/ p"`
  echo $right_line
}

case "$1" in
  left)
    echo " $(fg '#ff6d00')$bklyn_zsh_ostype "
    ;;
  center)
    shift
    #center_debug $@
    eval_center $@ 
    ;;
  right)
    case "$OSTYPE" in
      darwin*)
        osx_right
        ;;
      *)
        uptime_right
        ;;
    esac
    ;;
  *)
    echo "Unknown specifier $1"
    ;;
esac

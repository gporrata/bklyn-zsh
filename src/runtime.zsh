bklyn_zsh_days() {
  t=$(( $1 / (60 * 60 * 24) ))
  if (($t > 0)); then
    echo -n "$t days"
  fi
}
bklyn_zsh_hours() {
  t=$(( ($1 / (60 * 60)) % 24 ))
  if (($t > 0)); then
    echo -n "$t hours"
  fi
}
bklyn_zsh_minutes() {
  t=$(( ($1 / 60) % 60 ))
  if (($t > 0)); then
    echo -n "$t minutes"
  fi
}
bklyn_zsh_seconds() {
  t=$(( $1 % 60 ))
  if (( $t > 0 )); then
    echo -n "$t seconds"
  fi
}

bklyn_zsh_runtime_start() {
  bklyn_zsh_timer=$SECONDS
}

bklyn_zsh_runtime_end() {
  local tl=$(($SECONDS - ${bklyn_zsh_timer:-0}))
  if (( $tl >= 2 )); then
    local runtime_color=$(bklyn_zsh_fg1m '#ff7043')
    local total_time=($(bklyn_zsh_days $tl) $(bklyn_zsh_hours $tl) $(bklyn_zsh_minutes $tl) $(bklyn_zsh_seconds $tl))
    echo "$runtime_color runtime: $total_time"
  fi
  unset bklyn_zsh_timer
}

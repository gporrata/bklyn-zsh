bklyn_zsh_eval_prompt() {
  local last_status=$?
  if (( $last_status != 0 )); then
    local last_status_color=$(bklyn_zsh_fg1m '#ff5252')
    last_status_color=$(bklyn_zsh_encase $last_status_color)
    local last_status_icon=$'\uf00d'
    last_status=$last_status_color$last_status_icon' '$last_status$'\n'
  else
    last_status=''
  fi
  local dir_color=$(bklyn_zsh_fg1m '#9e9e9e')
  dir_color=$(bklyn_zsh_encase $dir_color)
  local prompt_color=$(bklyn_zsh_fg256 255)
  prompt_color=$(bklyn_zsh_encase $prompt_color)
  local time_color=$(bklyn_zsh_fg1m '#ffee58')
  time_color=$(bklyn_zsh_encase $time_color)
  local dharma_color=$(bklyn_zsh_fg1m '#2980B9')
  dharma_color=$(bklyn_zsh_encase $dharma_color)

  local bklyn_zsh_prompt=$'\uf489'
  local omph='ༀམཉཕདྷཧཱུཾ'
  local kchenno='ཀརྨ་པ་མཁྱེན་ནོ།'

  local git_status_length=$(( `tput cols` - 30 - 12 ))
  local git_status=`bklyn_zsh_git_status $git_status_length`
  local prompt_array=(
    $last_status
    ${bklyn_zsh_ostype_color}${bklyn_zsh_ostype}' '
    "$(bklyn_zsh_ssh_status)"
    $dir_color"%30<..<%~%<<"' '
    "$git_status"
    $'\n'$prompt_color$bklyn_zsh_prompt' '
  )
  PROMPT=${(j::)prompt_array}
  #RPROMPT="$dharma_color$kchenno $time_color%D{%H:%M} "
  RPROMPT="$time_color%D{%H:%M} "
}


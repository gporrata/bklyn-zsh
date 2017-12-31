bklyn_zsh_eval_prompt() {
  dir_color=$(bklyn_zsh_fg1m '#7F8C8D')
  dir_color=$(bklyn_zsh_encase $dir_color)
  prompt_color=$(bklyn_zsh_fg256 255)
  prompt_color=$(bklyn_zsh_encase $prompt_color)
  time_color=$(bklyn_zsh_fg1m '#ffca28')
  time_color=$(bklyn_zsh_encase $time_color)
  dharma_color=$(bklyn_zsh_fg1m '#2962ff')
  dharma_color=$(bklyn_zsh_encase $dharma_color)

  bklyn_zsh_prompt=$'\uf489'
  omph='ༀམཉཕདྷཧཱུཾ'
  kchenno='ཀརྨ་པ་མཁྱེན་ནོ།'

  git_status_length=$(( `tput cols` - 30 - 12 ))
  git_status=$(bklyn_zsh_git_status $git_status_length)
  prompt_array=(
    ${bklyn_zsh_ostype_color}${bklyn_zsh_ostype}
    ${dir_color}"%30<..<%~%<<"
    ${git_status}
    $'\n'${prompt_color}${bklyn_zsh_prompt}' '
  )

  PROMPT=${(j: :)prompt_array}
  RPROMPT="$dharma_color$kchenno $time_color%D{%H:%m} "
}


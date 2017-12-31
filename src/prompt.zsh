# color codes
# https://web.archive.org/web/20120905043337/http://lucentbeing.com/blog/that-256-color-thing

# man zshmisc for:
#   PROMPT codes
#     

bklyn_zsh_dir_color=$(bklyn_zsh_fg1m '#7F8C8D')
bklyn_zsh_dir_color=$(bklyn_zsh_encase $bklyn_zsh_dir_color)
bklyn_zsh_prompt_color=$(bklyn_zsh_fg256 255)
bklyn_zsh_prompt_color=$(bklyn_zsh_encase $bklyn_zsh_prompt_color)
bklyn_zsh_prompt=$'\uf489'

bklyn_zsh_new_line() {
  echo
}

eval_prompt() {
  #example_git='CORESVCS_2415_this_is_some_branch_that_i_want_to_show_and_it_is_very_very_long'
  example_git='CORESVCS_2415_this'

  bklyn_zsh_prompt_array=(
    ${bklyn_zsh_ostype_color}${bklyn_zsh_ostype}
    ${bklyn_zsh_dir_color}"%30<..<%~"
    $(bklyn_zsh_git_status)
    $'\n'${bklyn_zsh_prompt_color}${bklyn_zsh_prompt}' '
  )

  PROMPT=${(j: :)bklyn_zsh_prompt_array}
  RPROMPT='%D{%L:%m %p}'
}


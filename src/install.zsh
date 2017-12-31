bklyn_zsh_precmd_hook() {
  eval_prompt
}

bklyn_zsh_install() {
  [[ -z $precmd_functions ]] && precmd_functions=()
  precmd_functions=($precmd_functions bklyn_zsh_precmd_hook)
}

if [[ $bklyn_zsh_installed != 'installed' ]]; then
  bklyn_zsh_install
  bklyn_zsh_installed='installed'
fi

bklyn_zsh_precmd_hook() {
  bklyn_zsh_runtime_end
  bklyn_zsh_eval_prompt
}

bklyn_zsh_preexec_hook() {
  bklyn_zsh_runtime_start
}

[[ -z $precmd_functions ]] && precmd_functions=()

if [[ ${precmd_functions[(r)bklyn_zsh_precmd_hook]} != bklyn_zsh_precmd_hook ]]; then
  precmd_functions=($precmd_functions bklyn_zsh_precmd_hook)
fi

[[ -z $preexec_functions ]] && preexec_functions=()

if [[ ${preexec_functions[(r)bklyn_zsh_preexec_hook]} != bklyn_zsh_preexec_hook ]]; then
  preexec_functions=($preexec_functions bklyn_zsh_preexec_hook)
fi

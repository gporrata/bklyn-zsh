bklyn_zsh_script=${0:A}

# for testing; reload this quickly
rt() {
  source ${bklyn_zsh_script}
  echo source ${bklyn_zsh_script}
  time node ${bklyn_zsh_script:h}/dist/bklyn-zsh-bundle.js zsh-left `tput cols`
  time node ${bklyn_zsh_script:h}/dist/bklyn-zsh-bundle.js zsh-right `tput cols`
}

bklyn-zsh-preexec-hook() {
}

#
bklyn-zsh-precmd-hook() {
  cols=`tput cols`
  PROMPT=`node ${bklyn_zsh_script:h}/dist/bklyn-zsh-bundle.js zsh-left $cols`
  RPROMPT=`node ${bklyn_zsh_script:h}/dist/bklyn-zsh-bundle.js zsh-right $cols`
}

# make sure to install bklyn-zsh only once
if [[ $bklyn_zsh_installed != 'installed' ]]; then
  bklyn_zsh_installed='installed'

  [[ -z $preexec_functions ]] && preexec_functions=()
  preexec_functions=($preexec_functions bklyn-zsh-preexec-hook)

  [[ -z $precmd_functions ]] && precmd_functions=()
  precmd_functions=($precmd_functions bklyn-zsh-precmd-hook)
fi

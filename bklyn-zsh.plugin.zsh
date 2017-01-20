bklyn_zsh_script=${0:A}
bklyn_zsh_exec="node ${bklyn_zsh_script:h}/dist/bklyn-zsh-bundle.js"
export HOST=`hostname`

bklyn_zsh_dirtype() {
  if [[ -f 'package.json' ]]; then
    echo 'npm'
  elif [[ -d 'node_modules' ]]; then
    echo 'node'
  elif [[ -f 'build.sbt' ]]; then
    echo 'scala'
  elif [[ -f 'pom.xml' ]]; then
    echo 'maven'
  elif [[ -f 'build.xml' ]]; then
    echo 'java'
  elif [[ -f 'makefile' ]]; then
    echo 'cpp'
  elif [[ $PWD == $HOME ]]; then
    echo 'home'
  else
    echo ''
  fi
}

# for testing; reload this quickly
rt() {
  source ${bklyn_zsh_script}
  bklyn_zsh_cols=`tput cols`
  time BKLYN_ZSH_COLS=$bklyn_zsh_cols HOST=$HOST BKLYN_ZSH_DIRTYPE=`bklyn_zsh_dirtype` ${=bklyn_zsh_exec} zsh-left >/dev/null
  time BKLYN_ZSH_COLS=$bklyn_zsh_cols ${=bklyn_zsh_exec} zsh-right >/dev/null
}

bklyn-zsh-preexec-hook() {
}

#
bklyn-zsh-precmd-hook() {
  bklyn_zsh_cols=`tput cols`
  PROMPT=`BKLYN_ZSH_COLS=${bklyn_zsh_cols} HOST=$HOST BKLYN_ZSH_DIRTYPE=$(bklyn_zsh_dirtype) node ${bklyn_zsh_script:h}/dist/bklyn-zsh-bundle.js zsh-left --cols=$cols`
  RPROMPT=`BKLYN_ZSH_COLS=${bklyn_zsh_cols} node ${bklyn_zsh_script:h}/dist/bklyn-zsh-bundle.js zsh-right --cols=$cols`
}

# make sure to install bklyn-zsh only once
if [[ $bklyn_zsh_installed != 'installed' ]]; then
  bklyn_zsh_installed='installed'

  [[ -z $preexec_functions ]] && preexec_functions=()
  preexec_functions=($preexec_functions bklyn-zsh-preexec-hook)

  [[ -z $precmd_functions ]] && precmd_functions=()
  precmd_functions=($precmd_functions bklyn-zsh-precmd-hook)
fi

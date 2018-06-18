bklyn_zsh_fgr="%{\e[38m%}"
bklyn_zsh_bgr="%{\e[48m%}"

bklyn_zsh_fg256() {
  if [[ "$1" =~ "0x.+" ]]; then
    echo -ne "\e[38;5;$(( $1 ))m"
  else
    echo -ne "\e[38;5;$1m"
  fi
}

bklyn_zsh_bg256() {
  if [[ "$1" =~ "0x.+" ]]; then
    echo -ne "\e[48;5;$(( $1 + 0 ))m"
  else
    echo -ne "\e[48;5;$1m"
  fi
}

bklyn_zsh_fg1m() {
  if [[ "$1" =~ "#.+" ]]; then
    echo "\e[38;2;$(( 0x${1[2,3]} ));$(( 0x${1[4,5]} ));$(( 0x${1[6,7]} ))m"
  else
    echo -ne "\e[38;2;$1;$2;$3m"
  fi
}

bklyn_zsh_bg1m() {
  if [[ "$1" =~ "#.+" ]]; then
    echo "\e[38;2;$(( 0x${1[2,3]} ));$(( 0x${1[4,5]} ));$(( 0x${1[6,7]} ))m"
  else
    echo -ne "\e[38;2;$1;$2;$3m"
  fi
}

bklyn_zsh_encase() {
  echo -n "%{$1%}"
}

bklyn_zsh_reset=$(bklyn_zsh_encase `echo -ne "\e[0m"`)

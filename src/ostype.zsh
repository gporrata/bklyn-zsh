eval_bklyn_zsh_ostype() {
  case $OSTYPE in
    darwin*)
      echo -n $'\uf179'
      ;;
    linux*)
      echo -n $'\uf17c'
      #todo: determine the distribution here
      ;;
  esac
}

bklyn_zsh_ostype_color=$(bklyn_zsh_fg1m '#2980B9')
bklyn_zsh_ostype_color=$(bklyn_zsh_encase $bklyn_zsh_ostype_color)
bklyn_zsh_ostype=$(eval_bklyn_zsh_ostype)

bklyn_zsh_ssh_status() {
  if [[ "$SSH_TTY" != "" || "$SSH_CLIENT" != "" ]]; then
    local ssh_color=$(bklyn_zsh_fg1m '#66bb6a')
    ssh_color=$(bklyn_zsh_encase $ssh_color)
    local ssh_color2=$(bklyn_zsh_fg1m '#2e7d32')
    ssh_color2=$(bklyn_zsh_encase $ssh_color2)
    local at=$'\uf1fa'
    local ssh_status="$ssh_color%n $ssh_color2$at%m " 
    echo -n "$ssh_status"
  fi
}

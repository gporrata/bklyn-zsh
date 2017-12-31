bklyn_zsh_git_status() {
  local style
  typeset -A style
  style=(
    default '#1ABC9C'
    untracked '#f44336'
    unstaged '#f44336'
    staged '#F1C40F'
    branch_icon $'\ue725 '
    up_down_color '#D35400'
    up_icon $'\uf062'
    down_icon $'\uf063'
  )

  local is_git=true
  typeset -A changes
  changes=(unstaged false staged false untracked false)
  (git -c color.status=false status --porcelain=2 --branch 2>/dev/null ||
    echo '# not-a-branch' ) |
    while read line; do
      case $line in
        '# not-a-branch')
          is_git=false
          ;;
        '# branch.upstream'*)
          ;;
        '# branch.oid'*)
          ;;
        '# branch.head'*)
          local branch=${line:s/# branch.head //}
          branch=${branch##*/}
          local orig_branch_len=${#branch}
          branch=$branch[1,$(($1-2))]
          if (( ${#branch} != $orig_branch_len )); then
            branch="${branch}.."
          fi
          ;;
        '# branch.ab'*)
          [[ $line =~ '# branch.ab \+(.+) -(.+)' ]] &&
            local up=$match[1] &&
            local down=$match[2]
          ;;
        '?'*)
          changes+=(untracked true)
          ;;
        *)
          if [[ "$line" =~ '[0-9]+ \.[a-zA-Z0-9]' ]]; then
            changes+=(unstagled true)
          elif [[ "$line" =~ '[0-9]+ [a-zA-Z0-9].' ]]; then
            changes+=(staged true)
          fi
          ;;
      esac
    done

  if [[ $is_git == true ]]; then
    local show_up_down=true
    if [[ $changes[untracked] == true ]]; then
      local color=$style[untracked]
    elif [[ $changes[unstaged] == true ]]; then
      local color=$style[unstaged]
    elif [[ $changes[staged] == true ]]; then
      local color=$style[staged]
    else
      local color=$style[default]
    fi
    color=$(bklyn_zsh_fg1m $color)
    color=$(bklyn_zsh_encase $color)
    up_down_color=$(bklyn_zsh_fg1m $style[up_down_color])
    up_down_color=$(bklyn_zsh_encase $up_down_color)
    local git_line=$color$style[branch_icon]$branch$up_down_color

    if [[ $up != "" && $up != "0" ]]; then
      git_line="$git_line $style[up_icon]${up}"
    fi
    if [[ $down != "" && $down != "0" ]]; then
      git_line="$git_line $style[down_icon]${down}"
    fi
    echo -ne $git_line
  fi
}

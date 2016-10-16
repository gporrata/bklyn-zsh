# Configuracion POWERLVEL9K
POWERLEVEL9K_MODE='awesome-fontconfig'

########################
## COLORS + CONFIG
########################

# terminal id

(( term_idx = $(ps a | grep -c zsh) - 1 ))

# random os icon color
case $(( $term_idx % 5 )) in
  0)
    POWERLEVEL9K_OS_ICON_COLOR=166 # orange
    ;;
  1)
    POWERLEVEL9K_OS_ICON_COLOR=70 # green
    ;;
  2)
    POWERLEVEL9K_OS_ICON_COLOR=38 # blue
    ;;
  3)
    POWERLEVEL9K_OS_ICON_COLOR=white # white
    ;;
  4)
    POWERLEVEL9K_OS_ICON_COLOR=yellow
    ;;
esac

# color strategy
POWERLEVEL9K_COLOR_ACCENT=(black $POWERLEVEL9K_OS_ICON_COLOR 172 186 159)
POWERLEVEL9K_COLOR_SCHEME=(black 24 25 26 34 35 36 37)
#POWERLEVEL9K_COLOR_SCHEME=(232 235 238 239 240 242 244 246)

# prompt segments
POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(os_icon root_indicator context dir vcs)
POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS=(status load ram background_jobs time)
POWERLEVEL9K_STATUS_VERBOSE=true
POWERLEVEL9K_PROMPT_ON_NEWLINE=true
POWERLEVEL9K_RPROMPT_ON_NEWLINE=true

# os icon colors
POWERLEVEL9K_OS_ICON_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[1]
POWERLEVEL9K_OS_ICON_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[2]

# tmux winidx
POWERLEVEL9K_TMUX_WINIDX_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[1]
POWERLEVEL9K_TMUX_WINIDX_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[2]

# dont show user if the default
DEFAULT_USER=$USER

# context colors
POWERLEVEL9K_CONTEXT_DEFAULT_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[2]
POWERLEVEL9K_CONTEXT_DEFAULT_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[3]

# dir strategy
POWERLEVEL9K_SHORTEN_DIR_LENGTH=3
POWERLEVEL9K_SHORTEN_STRATEGY="truncate_left"

# dir colors
POWERLEVEL9K_DIR_HOME_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[3]
POWERLEVEL9K_DIR_HOME_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[4]
POWERLEVEL9K_DIR_HOME_SUBFOLDER_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[3]
POWERLEVEL9K_DIR_HOME_SUBFOLDER_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[4]
POWERLEVEL9K_DIR_DEFAULT_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[3]
POWERLEVEL9K_DIR_DEFAULT_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[4]

# `git hub colors`
POWERLEVEL9K_VCS_CLEAN_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[4]
POWERLEVEL9K_VCS_CLEAN_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[5]
POWERLEVEL9K_VCS_UNTRACKED_BACKGROUND='214'
POWERLEVEL9K_VCS_UNTRACKED_FOREGROUND='236'
POWERLEVEL9K_VCS_MODIFIED_BACKGROUND='196'
POWERLEVEL9K_VCS_MODIFIED_FOREGROUND='236'

# ram colors
POWERLEVEL9K_RAM_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[-3]
POWERLEVEL9K_RAM_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[1]

# load reset below
POWERLEVEL9K_LOAD_NORMAL_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[-2]
POWERLEVEL9K_LOAD_NORMAL_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[1]

# background jobs' colors
POWERLEVEL9K_BACKGROUND_JOBS_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[-1]
POWERLEVEL9K_BACKGROUND_JOBS_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[1]

# time configuration
POWERLEVEL9K_TIME_FORMAT="%D{\uf017 %H:%M \uf073 %d.%m}"

# time colors
POWERLEVEL9K_TIME_BACKGROUND=$POWERLEVEL9K_COLOR_SCHEME[8]
POWERLEVEL9K_TIME_FOREGROUND=$POWERLEVEL9K_COLOR_ACCENT[1]

########################
## ICONS FROM NERDS
########################

# no home or folder icons
POWERLEVEL9K_HOME_ICON=""
POWERLEVEL9K_HOME_SUB_ICON=""
POWERLEVEL9K_FOLDER_ICON=""

# line prefixes
POWERLEVEL9K_MULTILINE_FIRST_PROMPT_PREFIX=""
POWERLEVEL9K_MULTILINE_SECOND_PROMPT_PREFIX=" \uF489  " # prompt prompt
PS2=" \uF489  " # prompt prompt
PS3=" \uF18E  " # other prompt prompt
PS4=" \uF188  " # bug

# better ok icon
POWERLEVEL9K_OK_ICON="\uF42E "

# better ram icon
POWERLEVEL9K_RAM_ICON="\uF464 "

# better git icon
POWERLEVEL9K_VCS_GIT_ICON="\uF09B" # roundish git

segments_rounded() {
  POWERLEVEL9K_LEFT_SEGMENT_SEPARATOR="\uE0B4"
  POWERLEVEL9K_RIGHT_SEGMENT_SEPARATOR="\uE0B6"
  POWERLEVEL9K_LEFT_SUBSEGMENT_SEPARATOR="\uE0B5"
  POWERLEVEL9K_RIGHT_SUBSEGMENT_SEPARATOR="\uE0B7"
}

segments_in_flames() {
  POWERLEVEL9K_LEFT_SEGMENT_SEPARATOR="\uE0C0"
  POWERLEVEL9K_RIGHT_SEGMENT_SEPARATOR="\uE0C2"
}

segments_digital1() {
  POWERLEVEL9K_LEFT_SEGMENT_SEPARATOR="\uE0C4\uE0C4"
  POWERLEVEL9K_RIGHT_SEGMENT_SEPARATOR="\uE0C5\uE0C5"
}

segments_digital2() {
  POWERLEVEL9K_LEFT_SEGMENT_SEPARATOR="\uE0C6\uE0C6"
  POWERLEVEL9K_RIGHT_SEGMENT_SEPARATOR="\uE0C7\uE0C7"
}


case $(( $term_idx % 5 )) in
  1)
    segments_rounded
    ;;
  2)
    segments_in_flames
    ;;
  3)
    segments_digital1
    ;;
  4)
    segments_digital2
    ;;
esac

#######################
### SEGMENTS + FNS
#######################

# segment to show tmux window index in circle
# only available if tmux running
if { [ "$TERM" = "screen-256color" ] && [ -n "$TMUX" ]; } then
  winidx=$(tmux display-message -p '#I')
  if (( winidx > 20 )); then
    prompt_tmux_winidx() {
      $1_prompt_segment "$0" "$2" "$DEFAULT_COLOR" "blue" " $(winidx) " "#"
    }
  else
    circled_digits=$(printf %s \${$'\xEA',\`,{a..s}} | iconv -f UTF-16BE)
    prompt_tmux_winidx() {
      $1_prompt_segment "$0" "$2" "$DEFAULT_COLOR" "blue" " ${circled_digits:$winidx:1} " "#"
    }
  fi
else
  prompt_tmux_winidx() {}
fi

# fg colors
show_fg_colors() {
  for code ({000..255}) print -P -- "$code: %F{$code}This is how your text would look like%f"
}

# bg colors
show_bg_colors() {
  for code ({000..255}) print -P -- "$code: %K{$code}This is how your text would look like%f"
}

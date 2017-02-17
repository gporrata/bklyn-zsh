# bklyn-zsh

![screenshot](https://raw.githubusercontent.com/gporrata/bklyn-zsh/master/screenshot.png)

My own zsh theme, inspired by powerlevel9k and powerline. Requires:

* zplug ( you could use other zsh plugin managers but I haven't )
* rust + cargo
* git
* a nerd font

The screenshot above uses iTerm2, a nerd font, and solarized
colors. ust a few features including:

* show nice os icons for osx, linux (ubuntu or redhat)
* show nice icon for type of project language (js, scala, java, .c++, rust)
* show nice stats for current git branch / up / down / stashes. icon for unstaged
* show if last process failed
* show pid of zsh shell
* pretty colors defined in hex

Tmux Setup:

In your tmux.conf file do:
* source "$ZPLUG_HOME/repos/gporrata/bklyn_zsh/tmux-theme.conf"

Todo:

* show if in ssh
* show docker setup
* show puppet setup
* show version of project language (eg node ver, scala ver, java ver, gcc, rust ver, etc)
* provide tmux theme from same server, coming soon
* alternating powerline edges based on the day.

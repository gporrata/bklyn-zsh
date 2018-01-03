My ever evolving simple zsh & tmux theme using 100% zsh:

For Zsh:
* username & host when conntected over ssh
* os icon
* git branch when within git
* no. of git commits ahead
* no. of git commits behind
* no. of git stashes
* cute shell prompt
* time

For Tmux:
* os icon
* ifconfig up/down[link] rate stats from ifconfig on osx
* iostat stats on osx
* avg number of jobs in run queue over 1, 5, 15 mins from uptime otherwise
* time

Installation:

I use [zplug](https://github.com/zplug/zplug). Its fast, flexible, and easy. So
put a line in your zplugs file with something like

```zplug "gporrata/bklyn-zsh"```

To add to tmux add a line to your tmux.conf that looks like

```source $ZPLUG_HOME/repos/gporrata/bklyn-zsh/tmux-theme.conf```

This assumes that you have an environment variable ZPLUG_HOME set to where you
installed zplug.

You will also need a [nerd font](https://nerdfonts.com/) setup.

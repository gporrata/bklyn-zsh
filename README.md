# bklyn-zsh

![screenshot](https://raw.githubusercontent.com/gporrata/bklyn-zsh/master/screenshot.png)

My Powerline9k zsh theme inspired from other zsh themes. Features:

* better icons suited for nerd fonts
* random os icon color
* random theme for segment dividers:
  * regular triangular segments
  * rounded segments
  * in flame segments
  * digital1 segments
  * digital2 segments
* Nice prompt icon, even for PS2

__Left segment__: os_icon root_indicator context dir vcs

__Right segment__: status load ram background_jobs time

## Installation

You need zsh, [zplug](https://github.com/zplug/zplug), [powerlevel9k](https://github.com/bhilburn/powerlevel9k), and a [nerd font](https://github.com/ryanoasis/nerd-fonts). The screenshot above is using __nerd monofur font__ and __iterm2__. You could use some other zsh plugin manager but imho __zplug__ is the best. Also note you must install the theme before __powerlevel9k__. For instance, I use __nice:10__ when adding __powerlevel9k__ like so:

```bash
# bklyn-zsh
zplug "gporrata/bklyn-zsh"

# powerlevel 9k
zplug "bhilburn/powerlevel9k", nice:10
```

## TODO

* Segment for version of virtual environment when in a folder for that env.
* Submit to the good ppl at powerlevel9k.

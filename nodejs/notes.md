***

use precmd_functions to run before command prompt / alter prompt
use preexec_functions to run before executing command / 

eg.

something-preexec-hook() {
  
}

something-precmd-hook() {
  
}


[[ -z $preexec_functions ]] && preexec_functions=()
preexec_functions=($preexec_functions something-preexec-hook)

[[ -z $precmd_functions ]] && precmd_functions=()
precmd_functions=($precmd_functions something-precmd-hook)

***

use PROMPT to set left prompt
use RPROMPT to set right prompt

**

use `tput cols` to get determine what to display in PROMPT

**

might want to webpack this for performance, use node-babel for testing
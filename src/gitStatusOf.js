// @flow

import _ from 'lodash'

/* git output example

# branch.oid eb744800f30f6b55147fa2c3a2f3e2591f0ff2a3
# branch.head feature/revamp-using-node
# branch.upstream origin/feature/revamp-using-node
# branch.ab +0 -0
1 .M N... 100644 100644 100644 6015922c0254ff356df8f9f5e853c65753483afa 6015922c0254ff356df8f9f5e853c65753483afa bklyn-zsh.plugin.zsh
1 .M N... 100644 100644 100644 f9d35da32a052ac459972c4fd8c669166f43a3eb f9d35da32a052ac459972c4fd8c669166f43a3eb src/bklyn-zsh.js
1 .M N... 100644 100644 100644 93069613d3b23489b727033073121fa29681995f 93069613d3b23489b727033073121fa29681995f src/dirIcon.js
1 .M N... 100644 100644 100644 81b8b72531396449eb7ff26f8aeeec243d54bb8e 81b8b72531396449eb7ff26f8aeeec243d54bb8e src/icons.js
1 .M N... 100644 100644 100644 00a3532ff48684c0a790b3654fe0523521edee63 00a3532ff48684c0a790b3654fe0523521edee63 src/osIcon.js
? hi
? src/gitStatusOf.js

*/

export default (gitStatus /*: string */) => {
  if(gitStatus == '') {
    return ''
  }

  const [, branch] = /^\# branch\.head (.+)$/gm.exec(gitStatus) || []
  const [, up, down] = /^\# branch.ab \+(\d+) \-(\d+)/gm.exec(gitStatus) || []
  return `${branch} +${up} -${down}`
}

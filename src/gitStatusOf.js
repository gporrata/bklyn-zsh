// @flow

import _ from 'lodash'

import icons from './icons'
import scheme from './scheme'

/* example of: git -c color.status=false status --porcelain=2 --branch

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

----------

^branch.head (\w+) => branch name
^branch.ab +(\d+) -(\d+) => num commits ahead & behind
^? => file with untracked changes
^(\d+) .(\w) => staged changes
^(\d+) (\w). => unstaged changes
? => untracked file (will be considered to be unstaged change)
*/

const findStaging = (gitStatus) => {
  const re = /^((?:\?)|(?:\d+ ([\.\w])))/gm
  let match
  let staging = null
  while(match = re.exec(gitStatus)) {
    if(match[1] == '?' || (match[2] && match[2].startsWith('.'))) {
      return 'unstaged'
    }
    staging = 'staged'
  }
  return staging
}

export default (gitStatus /*: string */, gitStash /*: string */) => {
  if(gitStatus == '') {
    return ''
  }

  const [, branch] = /^\# branch\.head (.+)$/gm.exec(gitStatus) || []
  const [, up, down] = /^\# branch.ab \+(\d+) \-(\d+)/gm.exec(gitStatus) || []
  const staging = findStaging(gitStatus)
  const stashes = gitStash.split('\n').length - 1

  return _([
    icons.vcs.branch, ' ',
    staging && `${icons.vcs[staging]} `,
    scheme.vcs.fg1, branch, scheme.vcs.fg0,
    parseInt(up) && ` ${icons.vcs.up} ${scheme.vcs.fg1}${up}${scheme.vcs.fg0}`,
    parseInt(down) && ` ${icons.vcs.down} ${scheme.vcs.fg1}${down}${scheme.vcs.fg0}`,
    stashes && ` ${icons.vcs.stashes} ${scheme.vcs.fg1}${stashes}${scheme.vcs.fg0}`
  ]).filter().reduce((e, acc) => `${e}${acc}`)
  //return `${icons.vcs.branch} ${branch} +${up} -${down} stg ${staging} sta ${stashes}`
}

// @flow

import _ from 'lodash'
import osIcon from './osIcon'
import dirIcon from './dirIcon'
import icons from './icons'
import gitStatusOf from './gitStatusOf'

const debug = process.env.bklyn_zsh_debug == 'DEBUG' ? console.error : _.noop

//debug('BKLYN_ZSH_DIRTYPE='+process.env.BKLYN_ZSH_DIRTYPE)

const cols = parseInt(process.env.BKLYN_ZSH_COLS)
const gitStatus = _.defaultTo(process.env.BKLYN_ZSH_GIT, '')
const user = _.defaultTo(process.env.USER, '')
const host = _.defaultTo(process.env.HOST, '')
const pwd = _.defaultTo(process.env.PWD, '')

const combine = (...items) => {
  return _(items)
    .map(item => (typeof item == 'function') ? item() : item)
    .reduce((acc, item) => `${acc} ${item}`)
}

const left = () => {
  return combine(
    osIcon, `${user}@${host}`,
    dirIcon, pwd, gitStatusOf(gitStatus)
  ) + `\n${icons.prompt} `
}

const right = () => {
  return ''
}


switch(process.argv[2]) {
case 'zsh-left':
  console.log(left())
  break
case 'zsh-right':
  console.log(right())
  break
}

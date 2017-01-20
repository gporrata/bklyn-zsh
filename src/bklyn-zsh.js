import _ from 'lodash'
import osIcon from './osIcon'
import dirIcon from './dirIcon'
import icons from './icons'

const debug = process.env.bklyn_zsh_debug == 'DEBUG' ? console.error : _.noop

//debug('BKLYN_ZSH_DIRTYPE='+process.env.BKLYN_ZSH_DIRTYPE)

const cols = parseInt(process.env.BKLYN_ZSH_COLS)

const combine = (...items) => {
  return _(items)
    .map(item => (typeof item == 'function') ? item() : item)
    .reduce((acc, item) => `${acc} ${item}`)
}

const left = () => {
  return combine(
    osIcon, `${process.env.USER}@${process.env.HOST}`,
    dirIcon, process.env.PWD
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

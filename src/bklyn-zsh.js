// @flow

import 'babel-core/register'
import 'babel-polyfill'

//

import _ from 'lodash'
import koa from 'koa'
import route from 'koa-route'

const serverPort = parseInt(_.defaultTo(process.env.PORT, 90889))

koa()
  .use(route.get('/zsh-left', function *(next){
    this.body = 'TODO: zsh-left'
  }))
  .use(route.get('/zsh-right', function *(next) {
    this.body = 'TODO: zsh-right'
  }))
  .use(route.get('/tmux', function *(next) {
    this.body = 'TODO: tmux'
  }))
  .listen(serverPort, 'localhost')

console.log('running bklyn-zsh...')

/*
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

*/

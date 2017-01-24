// @flow

import 'babel-core/register'
import 'babel-polyfill'

import _ from 'lodash'
import koa from 'koa'
import route from 'koa-route'
import body from 'koa-better-body'
import yaml from 'js-yaml'
import styles from 'ansi-styles'

import icons from './icons'
import osIcon from './osIcon'
import gitStatusOf from './gitStatusOf'
import {dirIcon, dirTypeOf} from './dirTypeOf'
import scheme from './scheme'

const serverPort = parseInt(_.defaultTo(process.env.PORT, 90889))

const combine = (...items /*: Array<string> */) => items.join('')

const home = _.defaultTo(process.env.HOME, '')

const shortDir = (dir) => {
  return dir.startsWith(home)
    ? `~/${dir.slice(home.length + 1)}`
    : dir
}

const sep = icons.seps.digital[0]
//const sep = icons.seps.flames[0]
//const sep = icons.seps.angles[0]

const left = (data) => {
  return combine(
    // context
    scheme.os.bg, scheme.os.fg0,
    ' ', osIcon, ' ',
    scheme.os.fg1, data.USER, '@', data.HOST, ' ',
    scheme.os.bgAsFg, scheme.ssh.bg, sep,
    // ssh
    scheme.ssh.bg, scheme.ssh.fg0, icons.ssh,
    scheme.ssh.fg1, data.SSH_TTY, ' ', data.SSH_CLIENT, ' ',
    scheme.ssh.bgAsFg, scheme.dir.bg, sep,
    // dir
    scheme.dir.bg, scheme.dir.fg0,
    ' ', dirIcon(dirTypeOf(data.PWD)), ' ',
    scheme.dir.fg1, shortDir(data.PWD),
    scheme.dir.bgAsFg, scheme.vcs.bg, sep,
    // vcs
    scheme.vcs.bg, scheme.vcs.fg0,
    ' ', gitStatusOf(data.GIT, data.GIT_STASH), ' ',
    scheme.vcs.bgAsFg,
    styles.bgColor.close, sep,
    styles.color.close,
    // show prompt
    '\n', icons.prompt
  )
}

const right = (data) => {
  return combine(
    data.EXIT, ' ', data.PID
  )
}

/* input to posts

COLS: `tput cols`
PID: $!
EXIT: $?
PWD: $PWD
USER: $USER
HOST: $HOST
SSH_CLIENT: $SSH_CLIENT
SSH_TTY: $SSH_TTY
GIT: |
`git -c color.status=false status --porcelain=2 --branch | bklyn_zsh_yaml_pad`
GIT_STASH: |
`git stash list | bklyn_zsh_yaml_pad`

*/

koa()
  .use(body())
  .use(route.post('/zsh-left', function *(next){
    this.body = left(yaml.safeLoad(this.request.body))
  }))
  .use(route.post('/zsh-right', function *(next) {
    this.body = right(yaml.safeLoad(this.request.body))
  }))
  .use(route.post('/tmux', function *(next) {
    this.body = '...'
  }))
  .listen(serverPort, 'localhost')

console.log(`bklyn-zsh started on ${serverPort}`)

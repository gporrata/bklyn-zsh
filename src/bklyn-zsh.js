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

const combine = (...items /*: Array<string> */) => _(items).reduce((acc, item) => `${acc}${item}`)

const home = _.defaultTo(process.env.HOME, '')

const shortDir = (dir) => {
  return dir.startsWith(home)
    ? `~/${dir.slice(home.length + 1)}`
    : dir
}

const left = (data) => {
  return combine(
    // context
    scheme.os.bg, scheme.os.fg0,
    ' ', osIcon, ' ',
    scheme.os.fg1, data.USER, '@', data.HOST, ' ',
    // dir
    scheme.dir.bg, scheme.dir.fg0,
    ' ', dirIcon(dirTypeOf(data.PWD)), ' ',
    scheme.dir.fg1, shortDir(data.PWD),
    // vcs
    scheme.vcs.bg, scheme.vcs.fg0,
    ' ', gitStatusOf(data.GIT, data.GIT_STASH),
    styles.bgColor.close,
    styles.color.close
  ) + ` \n${icons.prompt} `
}

const right = (data) => {
  return combine(
    data.SSH_CLIENT, data.SSH_TTY,
    data.EXIT, data.PID
  )
}

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

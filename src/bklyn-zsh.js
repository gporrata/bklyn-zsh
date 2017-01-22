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

const combine = (...items) => {
  return _(items)
    .map(item => (typeof item == 'function') ? item() : item)
    .reduce((acc, item) => `${acc} ${item}`)
}

const left = (data) => {
  return combine(
    scheme.os.bg(
      osIcon, `${data.USER}@${data.HOST}`
    ),
    scheme.dir.bg(
      dirIcon(dirTypeOf(data.PWD)), data.PWD
    ),
    scheme.git.bg(
      gitStatusOf(data.GIT, data.GIT_STASH)
    )
  ) + `\n${icons.prompt} `
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

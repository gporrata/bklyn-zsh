// @flow

import 'babel-core/register'
import 'babel-polyfill'

import _ from 'lodash'
import koa from 'koa'
import route from 'koa-route'
import body from 'koa-better-body'
import yaml from 'js-yaml'

import icons from './icons'
import osIcon from './osIcon'
import gitStatusOf from './gitStatusOf'
import {dirIcon, dirTypeOf} from './dirTypeOf'

const serverPort = parseInt(_.defaultTo(process.env.PORT, 90889))

const combine = (...items) => {
  return _(items)
    .map(item => (typeof item == 'function') ? item() : item)
    .reduce((acc, item) => `${acc} ${item}`)
}

const left = (data) => {
  return combine(
    osIcon, `${data.USER}@${data.HOST}`,
    dirIcon(dirTypeOf(data.PWD)), data.PWD,
    gitStatusOf(data.GIT, data.GITSTASH),
    `${data.PID} ${data.EXIT}`
  ) + `\n${icons.prompt} `
}

koa()
  .use(body())
  .use(route.post('/zsh-left', function *(next){
    this.body = left(yaml.safeLoad(this.request.body))
  }))
  .use(route.post('/zsh-right', function *(next) {
    this.body = '...'
  }))
  .use(route.post('/tmux', function *(next) {
    this.body = '...'
  }))
  .listen(serverPort, 'localhost')

console.log(`bklyn-zsh started on ${serverPort}`)

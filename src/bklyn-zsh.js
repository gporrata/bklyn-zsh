// @flow

import 'babel-core/register'
import 'babel-polyfill'

import _ from 'lodash'
import koa from 'koa'
import route from 'koa-route'
import body from 'koa-better-body'
import yaml from 'js-yaml'
import stripAnsi from 'strip-ansi'

import contextSegment from './contextSegment'
import sshSegment from './sshSegment'
import dirSegment from './dirSegment'
import exitCodeSegment from './exitCodeSegment'
import pidSegment from './pidSegment'
import vcsSegment from './vcsSegment'
import {ansiZshProtect} from './ansiStringManipulation'
import {combineLeftSegments, combineRightSegments} from './segments'

const serverPort = parseInt(_.defaultTo(process.env.PORT, 90889))

const left = (data) =>
  combineLeftSegments(data.COLS,
    contextSegment(data.USER, data.HOST),
    sshSegment(data.SSH_TTY, data.SSH_CLIENT),
    dirSegment(data.PWD),
    vcsSegment(data.GIT, data.GIT_STASH)
  ) + '\n\uf489 '

const right = (data) =>
  combineRightSegments(
    exitCodeSegment(data.EXIT),
    pidSegment(data.PID)
  )

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
  .use(route.post('/left', function *(next){
    this.body = left(yaml.safeLoad(this.request.body))
  }))
  .use(route.post('/zsh-left', function *(next){
    this.body = ansiZshProtect(left(yaml.safeLoad(this.request.body)))
  }))
  .use(route.post('/right', function *(next) {
    this.body = right(yaml.safeLoad(this.request.body))
  }))
  .use(route.post('/zsh-right', function *(next) {
    this.body = ansiZshProtect(right(yaml.safeLoad(this.request.body)))
  }))
  .use(route.post('/tmux', function *(next) {
    this.body = '...'
  }))
  .listen(serverPort, 'localhost')

console.log(`bklyn-zsh started on ${serverPort}`)

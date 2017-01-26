import _ from 'lodash'
import path from 'path'
import fs from 'fs'

import {fg} from './segments'

const bg0 = '#094d77'
const fg0 = '#33a4ea'
const fg1 = '#ffffff'

const canAccess = (fname) => {
  try {
    fs.accessSync(fname, fs.F_OK)
    return true
  }
  catch(error) {
    return false
  }
}

const icons = {
  home: '\uf015',
  npm: '\ue71e',
  docker: '\ue7b0',
  java: '\ue738',
  cpp: '\ue51d',
  scala: '\ue706',
  node: '\ue7b2',
  js: '\ue74e',
  maven: '\ue738', // maven icon unrecognizable, use java instead '\ue7c4',
  other: '\uf07b',
}

const fileTypes = {
  'package.json': 'node',
  'node_modules': 'node',
  'build.sbt': 'scala',
  'pom.xml': 'maven',
  'build.xml': 'java',
  'makefile': 'cpp'
}

const dirIcon = (dirType) => {
  const dirIcon = icons[dirType]
  return dirIcon == undefined ? icons.other : dirIcon
}

const dirTypeOf = (dir, top = true) => {
  if(dir == '/') {
    return 'root'
  }
  else if(dir == process.env.HOME) {
    return top ? 'home' : 'other'
  }
  const is = _.find(fileTypes, (lang, fname) => canAccess(path.resolve(dir, fname)))
  if(is) {
    return is
  }
  return dirTypeOf(path.resolve(dir, '..'), false)
}

const home = _.defaultTo(process.env.HOME, '')

const shortDir = (dir) => {
  return dir.startsWith(home)
    ? `~/${dir.slice(home.length + 1)}`
    : dir
}

export default (pwd) => ({
  bg0, fg0,
  text: [
    dirIcon(dirTypeOf(pwd)), ' ', fg(fg1), shortDir(pwd)
  ].join('')
})

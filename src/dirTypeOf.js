import _ from 'lodash'
import path from 'path'
import fs from 'fs'
import icons from './icons'

const canAccess = (fname) => {
  try {
    fs.accessSync(fname, fs.F_OK)
    return true
  }
  catch(error) {
    cosole.log(error)
    return false
  }
}

const fileTypes = {
  'package.json': 'npm',
  'node_modules': 'node',
  'build.sbt': 'scala',
  'pom.xml': 'maven',
  'build.xml': 'java',
  'makefile': 'cpp'
}

export const dirIcon = (dirType) => {
  const dirIcon = icons.dirType[dirType]
  return dirIcon == undefined ? icons.dirType.other : dirIcon
}

export const dirTypeOf = (dir, top = true) => {
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

// @flow

import os from 'os'
import {fg} from './segments'

const bg0  = '#7490aa'
const fg0 = '#C0392B'
const fg1 = '#ffffff'

const icons = {
  darwin: '\uf179',
  freebsd: 'undefined',
  linux: '\uf17c',
    redhat: '\uf309',
    ubuntu: '\ue73a',
  win32: '\uf17a',
}

const calcOsIcon = () => {
  switch(os.platform()) {
    case 'darwin':
      return icons.darwin
    case 'freebsd':
      return icons.freebsd
    case 'linux':
      return icons.linux
    case 'win32':
      return icons.win32
    default:
      return ''
  }
}

const osIcon = calcOsIcon()

export default (user /*: string */, host /*: string */) => ({
  bg0, fg0,
  text: [
    osIcon, ' ', fg(fg1), user, '@', host
  ].join(''),
})

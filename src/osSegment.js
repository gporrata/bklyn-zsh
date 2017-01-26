// @flow

import os from 'os'
import {fg} from './segments'

const bg0 = '#094d77'
const fg0 = '#2ECC71'

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

export default () => ({
  bg0, fg0,
  text: [
    osIcon,
  ].join(''),
})

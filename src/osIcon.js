// @flow

import os from 'os'
import icons from './icons'

export default () => {
  let show = ''
  switch(os.platform()) {
    case 'darwin':
      show = icons.os.darwin
      break
    case 'freebsd':
      show = icons.os.freebsd
      break
    case 'linux':
      show = icons.os.linux
      break
    case 'win32':
      show = icons.os.win32
      break
    default:
      show =''
  }
  return show
}

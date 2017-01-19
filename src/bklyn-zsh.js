import _ from 'lodash'
import os from 'os'

const cols = parseInt(process.argv[3])

const icons = {
  'darwin': 'osx',
  'freebsd': 'freebsd',
  'linux': 'linux',
  'win32': 'win32',
}

const osPrompt = () => {
  let show = ''
  switch(os.platform()) {
    case 'darwin':
      show = icons.darwin
      break
    case 'freebsd':
      show = icons.freebsd
      break
    case 'linux':
      show = icons.linux
      break
    case 'win32':
      show = icons.win32
      break
    default:
      show =''
  }
  return show
}

const combine = (items) => {
  let result = ''
  items.forEach(item => {
    switch(typeof item) {
      case 'function':
        result += item()
        break
      case 'string':
        result += item()
        break
    }
  })
  return result
}

const left = () => {
  return combine([osPrompt])
}

const right = () => {
  return ''
}


switch(process.argv[2]) {
case 'zsh-left':
  console.log(left())
  break
case 'zsh-right':
  console.log(right())
  break
}

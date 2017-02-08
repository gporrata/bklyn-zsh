import {fg} from './segments'

const bg0 = '#D35400'
const fg0 = '#094d77'
const fg1 = '#bee3f7'

const clientIp = (client) => client.replace(/((?:\d+\.)+(?:\d+)).+/, '$1')

export default (tty, client, user, host) => {
  return (!tty && !client) ? {
    text: null
  } : {
    bg0, fg0,
    text: `\uf0c2  ${fg(fg1)}${user}@${host}`
  }
}

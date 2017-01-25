import {fg} from './segments'

const bg0 = '#843200'
const fg0 = '#D35400'
const fg1 = '#ffffff'

const clientIp = (client) => client.replace(/((?:\d+\.)+(?:\d+)).+/, '$1')

export default (tty, client) => {
  return (!tty && !client) ? {
    text: null
  } : {
    bg0, fg0,
    text: `\uf1be  ${fg(fg1)}${tty} ${clientIp(client)}`
  }
}

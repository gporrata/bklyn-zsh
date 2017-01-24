import {fg} from './segments'

const bg0 = '#843200'
const fg0 = '#D35400'
const fg1 = '#ffffff'

export default (tty, client) =>
  (!tty && !client)
    ?
    {
      text: null
    }
    :
    {
      bg0, fg0,
      text: `\uf1be  ${fg(fg1)}${tty} ${client}`
    }

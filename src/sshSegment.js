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
      text: `${fg(fg1)}\uf1be${fg(fg0)}  ${tty} ${client}`
    }

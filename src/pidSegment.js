import {fg} from './segments'

const bg0 = '#BDC3C7'
const fg0 = '#808589'
const fg1 = '#3d3e3f'

export default (pid) => {
  if(!pid) {
    return {
      text: null
    }
  }
  else {
    const text = `\uf12e ${fg(fg1)}${pid}${fg(fg0)}`
    return {text, bg0, fg0, fg1}
  }
}

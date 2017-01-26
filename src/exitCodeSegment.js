import {fg} from './segments'

const fgSuccess = '#033d1a'
const fgFailed = '#C0392B'
const bg0 = '#ECF0F1'
const fg0 = '#094d77'
const fg1 = '#000000'

export default (exitCode) => {
  if(!exitCode && exitCode != 0) {
    return {
      text: null
    }
  }
  else {
    const text = (exitCode == 0) ? `${fg(fgSuccess)}\uf00c${fg(fg0)}  ` : `${fg(fgFailed)}\uf00d${fg(fg1)} ${exitCode} `
    return {text, bg0, fg0}
  }
}

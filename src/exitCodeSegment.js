import icons from './icons'
import scheme from './scheme'

export default (exitCode) => {
  if(!exitCode && exitCode != 0) {
    return {
      text: null
    }
  }
  else {
    const text = (exitCode == 0) ? '\uf00c  ' : `\uf00d ${exitCode} `
    return {
      text,
      bg: '#ffffff',
      fg0: '#000000',
    }
  }
}

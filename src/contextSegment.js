const bg0 = '#1a2d3f'
const fg0 = '#34495E'
const fg1 = '#ffffff'

export default (user, host) => {
  const text = [
    user, '@', host
  ].join('')
  return {text, bg0, fg0}
}

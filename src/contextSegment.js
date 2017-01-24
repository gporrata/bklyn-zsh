const bg0 = '#1a2d3f'
const fg0 = '#34495E'
const fg1 = '#ffffff'

export default (user, host) => ({
  bg0, fg0,
  text: [
    user, '@', host
  ].join(''),
})

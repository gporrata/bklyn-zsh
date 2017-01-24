export default (pid) => {
  if(!pid) {
    return {
      text: null
    }
  }
  else {
    return {
      text: `\uf12e ${pid}`,
      bg: '#ffffff',
      fg0: '#000000',
    }
  }
}

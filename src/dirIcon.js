import icons from './icons'

export default () => {
  const dirIcon = icons.dirType[process.env.BKLYN_ZSH_DIRTYPE]
  return dirIcon == undefined ? icons.dirType.other : dirIcon
}

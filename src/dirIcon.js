// @flow

import _ from 'lodash'
import icons from './icons'

export default () => {
  const dirIcon = icons.dirType[_.defaultTo(process.env.BKLYN_ZSH_DIRTYPE, 'other')]
  return dirIcon == undefined ? icons.dirType.other : dirIcon
}

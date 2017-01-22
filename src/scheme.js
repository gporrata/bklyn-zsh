// @flow

import styles from 'ansi-styles'

const bg = (color) => styles.bgColor.ansi16m.hex(color)
const fg = (color) => styles.color.ansi16m.hex(color)

export default {
  os: {
    bg: bg('#1a2d3f'),
    bgAsFg: fg('#1a2d3f'),
    fg0: fg('#34495E'),
    fg1: fg('#ffffff')
  },
  dir: {
    bg: bg('#094d77'),
    bgAsFg: fg('#094d77'),
    fg0: fg('#2980B9'),
    fg1: fg('#ffffff')
  },
  vcs: {
    bg: bg('#033d1a'),
    bgAsFg: fg('#033d1a'),
    fg0: fg('#27AE60'),
    fg1: fg('#ffffff'),
    fg2: fg('#F1C40F'),
    fgUnstaged: fg('#E74C3C'),
    fgStaged: fg('#E67E22'),
  }
}

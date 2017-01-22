import styles from 'ansi-styles'

const setBg = (bg) => (...text) => `${styles.bgColor.ansi16m.hex(bg)}${text}${styles.bgColor.close}`

export default {
  os: {
    bg: setBg('#34495E')
  },
  dir: {
    bg: setBg('#2980B9')
  },
  git: {
    bg: setBg('#27AE60')
  }
}

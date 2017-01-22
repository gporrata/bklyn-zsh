import styles from 'ansi-styles'

const styleWith = (fg, bg) => (...text) => `${styles.color.ansi16m.hex(fg)}${styles.bgColor.ansi16m.hex(bg)}${text}${styles.color.close}${styles.bgColor.close}`

export default {
  os: {
    default: styleWith('#000000', '#34495E')
  },
  dir: {
    default: styleWith('#000000', '#2980B9')
  },
  git: {
    default: styleWith('#000000', '#27AE60')
  }
}

import _ from 'lodash'
import styles from 'ansi-styles'

export const bg = (color) => color ? styles.bgColor.ansi16m.hex(color) : ''
export const fg = (color) => color ? styles.color.ansi16m.hex(color) : ''

const seps = {
  angles: ['\ue0b0', '\ue0b1', '\ue0b2', '\ue0b3'],
  curves: ['\ue0b4', '\ue0b5', '\ue0b6', '\ue0b7'],
  flames: ['\ue0c0', '\ue0c1', '\ue0c2', '\ue0c3'],
  digital: ['\ue0c4', '\ue0c5', '\ue0c6', '\ue0c7'],
  // sorry, not a fan of the other seps
}

export const combineLeftSegments = (cols, segments) => {
  _(segments)
  .filter(!segments.text)
  .reduce((acc, segment, index, coll) => {
    if(acc.lineLength == 0) {
      return {
        text: acc.text + segment.text,
        lineLength: segment.text.length,
        priorSegment: segment
      }
    }
    else if(acc.lineLength + acc.text.length < cols) {
      return {
        text: acc.text + leftSep(priorSegment, segment) + segment.text,
        lineLength: acc.lineLength + segment.text.length,
        priorSegment: segment
      }
    }
    else {
      return {
        text: acc.text + leftSep(priorSegment, undefined) + '\n' + segment.text,
        lineLength: 0,
        priorSegment: segment
      }
    }
  },
  {
    text: '',
    lineLength: 0,
    priorSegment: null
  })
}

const rightSep = (sega, segb) => {
  return [fg(segb.bg0), bg(sega && sega.bg0), seps.digital[2], fg(segb.fg0), bg(segb.bg0)].join('')
}

export const combineRightSegments = (...segments) => {
  return [
    _(segments)
      .filter(segment => segment.text)
      .reduce((acc, segment, index, coll) => ({
        text: [
          acc.text,
          rightSep(acc.priorSegment, segment),
          segment.text
        ].join(''),
        priorSegment: segment
      }),
      {
        text: '',
        priorSegment: null
      })
      .text,
    styles.color.close,
    styles.bgColor.close,
  ].join('')
}

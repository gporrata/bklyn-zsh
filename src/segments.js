import _ from 'lodash'
import styles from 'ansi-styles'

const bg = (color) => styles.bgColor.ansi16m.hex(color)
const fg = (color) => styles.color.ansi16m.hex(color)

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
  //return [fg(segb.bg), bg(segb.bg)].join()
  return ''
}

export const combineRightSegments = (...segments) => {
  return _(segments)
    .filter(segments.text)
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
    .text
}

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

const style = 'curves'

const colorSeg = (seg) => [fg(seg.fg0), bg(seg.bg0)].join('')

const leftSep = (sega, sebg) => {

}

export const combineLeftSegments = (cols, ...segments) => [
  _(segments)
  .filter(segment => segment.text)
  .reduce((acc, segment, index, coll) => {
    if(acc.lineLength == 0) {
      return {
        text: [
          acc.text,
          colorSeg(segment),
          segment.text
        ].join(''),
        lineLength: segment.text.length,
        priorSegment: segment
      }
    }
    else if(acc.lineLength + acc.text.length < cols) {
      return {
        text: [
          acc.text,
          leftSep(acc.priorSegment, segment),
          colorSeg(segment),
          segment.text
        ].join(''),
        lineLength: acc.lineLength + segment.text.length,
        priorSegment: segment
      }
    }
    else {
      return {
        text: [
          acc.text,
          leftSep(acc.priorSegment, undefined),
          '\n',
          colorSeg(segment),
          segment.text
        ].join(''),
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
  .text,
  styles.color.close,
  styles.bgColor.close,
].join('')


const rightSep = (sega, segb) => {
  return [fg(segb.bg0), bg(sega && sega.bg0), seps[style][2], fg(segb.fg0), bg(segb.bg0)].join('')
}

export const combineRightSegments = (...segments) => [
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

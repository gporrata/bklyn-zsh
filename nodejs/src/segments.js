import _ from 'lodash'
import styles from 'ansi-styles'

import {ansiStrip} from './ansiStringManipulation'

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

const separateLeftSegments = (sega, segb) =>
  sega && segb && sega.bg0 != segb.bg0 ?
  [
    fg(sega.bg0),
    bg(segb.bg0),
    seps[style][0],
  ].join('') : ''

export const combineLeftSegments = (cols, ...segments) => {
  const fsegments = _(segments).filter(segment => segment.text)
  return [
    // all segments combined
    fsegments
      .map((segment, index, coll) => [
        separateLeftSegments(coll[index - 1], segment),
        bg(segment.bg0),
        fg(segment.fg0),
        ' ', segment.text,
      ].join(''))
      .value()
      .join(''),
    ' ',
    fg(fsegments.last().bg0),
    styles.bgColor.close,
    seps[style][0],
    styles.bgColor.close,
    styles.color.close,
  ].join('')
}





const rightSep = (sega, segb) => [
  fg(segb.bg0), bg(sega && sega.bg0), seps[style][2], fg(segb.fg0), bg(segb.bg0)
].join('')

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

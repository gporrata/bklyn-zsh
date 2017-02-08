export const ansiStrip = (text) => text.replace(/(\x1b\[(?:\d+;)*\d+m)/gm, '')

export const ansiZshProtect = (text) => text.replace(/(\x1b\[(?:\d+;)*\d+m)/g, '%{$1%}')

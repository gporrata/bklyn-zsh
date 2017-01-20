export default {
  // os
  os: {
    darwin: '\uf179',
    freebsd: 'undefined',
    linux: '\uf17c',
      redhat: '\uf309',
      ubuntu: '\ue73a',
    win32: '\uf17a',
  },

  // browser; not sure if we need this
  chrome: '\uf268',
  firefox: '\uf269',
  explorer: '\uf26b',

  // directories
  dirType: {
    home: '\uf015',
    npm: '\ue71e',
    docker: '\ue7b0',
    java: '\ue738',
    cpp: '\ue51d',
    scala: '\ue706',
    node: '\ue719',
    js: '\ue74e',
    maven: '\ue738', // maven icon unrecognizable, use java instead '\ue7c4',
    other: '\uf07b',
  },

  // process stats
  success: '\uf00c', // check, better might be '/uf046', '\uf058', or ''\uf05d'
  failed: '\uf00d', // X, better might be '\uf057', '\uf05c'

  // vnc
  github: '\uf113', // better might be '\uf1d2' or '\uf1d3'
  branch: '\uf126',

  // stats
  floppy: '\uf0c7',
  graph: '\uf012',
  piechart: '\uf200',
  power: '\uf1e6',
  settings: '\uf013',
  bluetooth: '\uf294',
  pulse: '\uf46a',

  // battery power
  battery: ['\uf240', '\uf241', '\uf242', '\uf243', '\uf244'],

  // ssh
  ssh: '\uf1be', // cloud, better might be '\uf289', or '\ue7ae'

  // separators, all from powerline
  styles: {
    angles: ['\ue0b0', '\ue0b1', '\ue0b2', '\ue0b3'],
    curves: ['\ue0b4', '\ue0b5', '\ue0b6', '\ue0b7'],
    flames: ['\ue0c0', '\ue0c1', '\ue0c2', '\ue0c3'],
    digital: ['\ue0c4', '\ue0c5', '\ue0c6', '\ue0c7'],
    // sorry, not a fan of the other styles
  },

  // misc
  prompt: '\uf489', // or '\ue795' or '\uf00d' (keyboard)
  fire: '\uf490',
  at: '\ue748 ',
  person: '\uf1ae'
}

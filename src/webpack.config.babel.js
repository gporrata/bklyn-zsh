import path from 'path'
import nodeExternals from 'webpack-node-externals'

const rules = [
  // js
  {
    test: /\.js$/,
    loader: 'babel-loader',
    exclude: /node_modules/, // exclude node_modules from being babel-ed|parsed,
    options: {
      "presets": [
        "latest"
      ]
    }
  },
]

export default {
  entry: {
    script: path.resolve(__dirname, './bklyn-zsh')
  },
  output: {
    path: path.resolve(__dirname, '../dist'),
    filename: 'bklyn-zsh-bundle.js',
    libraryTarget: 'commonjs2'
  },
  module: {
    rules
  },
  resolve: {
    modules: [
      'node_modules'
    ],
    extensions: ['.js', '.json']
  },
  devtool: 'source-map',
  target: 'node',
  externals: [nodeExternals()]
}

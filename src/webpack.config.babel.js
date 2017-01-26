import path from 'path'
import nodeExternals from 'webpack-node-externals'
import webpack from 'webpack'

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

const plugins = process.env.NODE_ENV === 'production' ? [
  new webpack.optimize.UglifyJsPlugin({
    compress: {
      screw_ie8: true,
      warnings: false
    },
    mangle: {
      screw_ie8: true
    },
    output: {
      comments: false,
      screw_ie8: true
    }
  })
] : undefined

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
  plugins,
  devtool: 'source-map',
  target: 'node',
  externals: [nodeExternals()]
}

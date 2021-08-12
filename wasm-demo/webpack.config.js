const path = require('path')
const HtmlPlugin = require('html-webpack-plugin')

module.exports = {
  entry: './index.js',
  output: {
    path: path.join(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  plugins: [
    new HtmlPlugin({
      title: 'wasm demo',
    }),
  ],
  mode: 'development',
  devtool: 'cheap-module-source-map',
  experiments: {
    asyncWebAssembly: true,
  },
}

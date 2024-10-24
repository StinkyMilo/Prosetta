const path = require('path');

module.exports = {
  entry: './script.js',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist'),
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        exclude: /node_modules/,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.svg$/,
        exclude: /node_modules/,
        loader: 'svg-inline-loader'
      },
    ],
  },
  mode: 'production',
  devtool: 'source-map',
};

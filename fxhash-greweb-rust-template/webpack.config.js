const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: process.env.BUILD_MODE || "production",
  module: {
    rules: [
      {
        test: /\.glsl$/,
        use: 'webpack-glsl-minify'
      },
    ],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "index.html", },
        {
          from: "static/*",
          to: "[name][ext]",
          info: {
            minimized: true // tells plugin to not minimize the file
          }
        },
      ],
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.'),
    }),
  ],
  experiments: {
    syncWebAssembly: true,
  },
};
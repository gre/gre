const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  mode: "production",
  entry: "./index.js", // input file of the JS bundle
  output: {
    filename: "bundle.js", // output filename
    path: path.resolve(__dirname, "dist"), // directory of where the bundle will be created at
  },
  plugins: [
    new WasmPackPlugin({
      forceMode: "production",
      crateDirectory: path.resolve(__dirname, "rust"), // Define where the root of the rust code is located (where the cargo.toml file is located)
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.m?js$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/preset-env", "@babel/preset-react"],
          },
        },
      },
    ],
  },
};

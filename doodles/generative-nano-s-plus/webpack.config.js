const path = require("path");
module.exports = {
  plugins: [],
  entry: {
    main: path.join(__dirname, "./entry.js"),
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    libraryTarget: "umd",
  },
  externals: {},
  module: {
    rules: [
      {
        test: /\.m?js$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/preset-env"],
          },
        },
      },
    ],
  },
};

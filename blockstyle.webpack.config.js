
module.exports = {
    plugins: [],
    entry: {
      main: "./blockarts/exportbundle.js"
    },
    output: {
      libraryTarget: "commonjs"
    },
    externals: {
        react: "react",
        resolve: {
            react: require("react"),
        }
    },
    module: {
      rules: [
        {
          test: /\.m?js$/,
          exclude: /(node_modules|bower_components)/,
          use: {
            loader: "babel-loader",
            options: {
                "presets": ["@babel/preset-env", "@babel/preset-react"]
            }
          }
        }
      ]
    }
  };
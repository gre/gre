set -e
NAME=$1
webpack --mode development --config main.webpack.config.js
cp src/index.html dist
cp -R static/ dist/static/ || true 2> /dev/null

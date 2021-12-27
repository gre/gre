set -e
NAME=$1
rm -rf dist; mkdir dist;
NODE_ENV=production webpack --mode production --config doodles/$NAME/main.webpack.config.js
cp doodles/$NAME/index.html dist
rm -f dist/*.wasm && yarn zip-dist

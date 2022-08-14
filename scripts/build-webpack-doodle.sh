set -e
set -x
NAME=$1
rm -rf dist; mkdir dist;
NODE_ENV=production webpack --mode production --config doodles/$NAME/main.webpack.config.js
cp doodles/$NAME/index.html dist
cp -R doodles/$NAME/static/ dist/static/ || true 2> /dev/null
cp doodles/$NAME/rust/src/lib.rs dist/code.rs
rm -f dist/*.wasm && yarn zip-dist

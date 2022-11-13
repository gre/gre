set -e
set -x
NAME=$1
rm -rf dist; mkdir dist;
NODE_ENV=production webpack --mode production --config doodles/$NAME/main.webpack.config.js
cp doodles/$NAME/index.html dist
cp -R doodles/$NAME/static/ dist/static/ || true 2> /dev/null
if [ -f doodles/$NAME/rust/src/lib.rs ] ; then 
  cp doodles/$NAME/rust/src/lib.rs dist/code.rs
fi
rm -f dist/*.wasm && yarn zip-dist

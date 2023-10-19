set -e
set -x
NAME=$1
rm -rf dist; mkdir dist;
NODE_ENV=production webpack --mode production --config doodles/$NAME/main.webpack.config.js
cp doodles/$NAME/index.html dist
if [ -f doodles/$NAME/fxhash.js ] ; then
  cp doodles/$NAME/fxhash.js dist
fi
if [ -x doodles/$NAME/static ] ; then
  cp -R doodles/$NAME/static/ dist/static/
fi
if [ -f doodles/$NAME/rust/src/lib.rs ] ; then 
  cp doodles/$NAME/rust/src/lib.rs dist/code.rs
fi
rm -f dist/*.wasm && yarn zip-dist

set -e
set -x
NAME=$1
rm -rf dist; mkdir dist;
NODE_ENV=production webpack --mode production --config main.webpack.config.js
cp src/index.html dist
cp -R static/ dist/static/ || true 2> /dev/null
cp rust/src/lib.rs dist/code.rs
rm -f dist/*.wasm
rm -f dist.zip
cd dist
zip -r ../dist.zip *

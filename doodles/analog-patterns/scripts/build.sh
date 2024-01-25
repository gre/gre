set -e
set -x

MODE=${1:-"production"}

if [ "$MODE" = "production" ]; then
  rm -rf dist dist.zip
  mkdir dist
fi

wasm-pack build --target web . --release
NODE_ENV=production webpack --mode $MODE --config webpack.config.js

if [ "$MODE" = "production" ]; then
  cp -R static/* dist/.
  cp src/lib.rs dist/code.rs
  cp index.html dist
  cd dist
  zip -r ../dist.zip *
fi
set -e
set -x
NAME=$1
webpack --mode development --config doodles/$NAME/main.webpack.config.js
cp doodles/$NAME/index.html dist
cp doodles/$NAME/*.ttf dist || true 2> /dev/null
cp -R doodles/$NAME/static/ dist/static/ || true 2> /dev/null


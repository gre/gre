set -e
NAME=$1
webpack --mode development --config doodles/$NAME/main.webpack.config.js
cp doodles/$NAME/index.html dist
cp doodles/$NAME/*.ttf dist


set -e
NAME=$1
wasm-pack build --target web doodles/$NAME/rust --debug

set -e
NAME=$1
wasm-pack build --target web rust --release

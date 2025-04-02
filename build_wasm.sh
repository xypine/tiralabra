set -x # Print all commands we run
wasm-pack build --target web
rm -rf ./frontend/pkg
cp -r ./pkg ./frontend/pkg

set -x # Print all commands we run

cargo doc
cp -r ./target/doc/* ./frontend/public/docs/

wasm-pack build --release --target web
rm -rf ./frontend/pkg
cp -r ./pkg ./frontend/pkg
rm -rf ./pkg

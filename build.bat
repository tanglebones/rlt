cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target\wasm32-unknown-unknown\release\rlt.wasm --out-dir dist --no-modules --no-typescript

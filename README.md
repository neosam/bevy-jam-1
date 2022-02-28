

cargo build --target wasm32-unknown-unknown --release

wasm-bindgen --out-dir docs/game --out-name unfair-pong --target web --no-typescript target/wasm32-unknown-unknown/release/unfair-pong.wasm

cargo install -f wasm-bindgen-cli --version 0.2.79
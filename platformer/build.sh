cargo build --release
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm > cart.min.wasm
wasm-opt -Oz --zero-filled-memory --strip-producers --dce cart.min.wasm -o cart.min.wasm
w4 bundle cart.min.wasm --title "mygame" --html ../docs/mygame.html

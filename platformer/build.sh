cargo build --release
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm > target/wasm32-unknown-unknown/release/cart.min.wasm
npx wasm-opt -Oz --zero-filled-memory --strip-producers --dce target/wasm32-unknown-unknown/release/cart.min.wasm -o target/wasm32-unknown-unknown/release/cart.min.wasm
w4 bundle target/wasm32-unknown-unknown/release/cart.min.wasm --title "mygame" --html ../docs/mygame.html

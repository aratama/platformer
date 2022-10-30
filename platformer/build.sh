cargo build --release
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm > cart.min.wasm
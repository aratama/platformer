# Platformer

### Build

```bash
cargo build --release
```

### Run locally

```bash
w4 run target/wasm32-unknown-unknown/release/cart.wasm
```

```bash
w4 watch target/wasm32-unknown-unknown/release/cart.wasm
```

https://github.com/aduros/wasm4/issues/238

```
$ cargo install wasm-snip
$ wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm > cart.min.wasm
$ w4 run cart.min.wasm
```

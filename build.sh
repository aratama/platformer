rm -r target
cargo build --release

# snip-rust-fmt-code を指定しているので to_string や format! が動かない
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm > target/wasm32-unknown-unknown/release/towerclimber.wasm
npx wasm-opt -Oz --zero-filled-memory --strip-producers --dce target/wasm32-unknown-unknown/release/towerclimber.wasm -o target/wasm32-unknown-unknown/release/towerclimber.wasm

# Error: Section metadata not found
# npx @webassemblyjs/wasm-strip target/wasm32-unknown-unknown/release/towerclimber.wasm

ls -l target/wasm32-unknown-unknown/release/cart.wasm
ls -l target/wasm32-unknown-unknown/release/towerclimber.wasm
w4 bundle target/wasm32-unknown-unknown/release/towerclimber.wasm --title "Tower Climber" --html ./docs/index.html

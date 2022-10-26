#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

const BUFFER_SIZE: usize = 13000;

#[no_mangle]
fn update() {
    let buf: [u8; BUFFER_SIZE] = [255; BUFFER_SIZE];
    let vec: Vec<u8> = vec![255; 32768];
    let vec2: Vec<u8> = vec![255; 4068];
}

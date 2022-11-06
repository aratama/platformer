#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

const BUFFER_SIZE: usize = 13000;

#[no_mangle]
fn update() {
    let _: [u8; BUFFER_SIZE] = [255; BUFFER_SIZE];
    let _: Vec<u8> = vec![255; 32768];
    let _: Vec<u8> = vec![255; 4068];
}

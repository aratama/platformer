// ladder
use crate::wasm4::*;
use crate::image::Image;

const LADDER_WIDTH: u32 = 8;
const LADDER_HEIGHT: u32 = 16;
const LADDER_FLAGS: u32 = BLIT_2BPP;
const LADDER: [u8; 32] = [ 0x2e,0xa8,0x20,0x08,0x2e,0xa8,0x20,0x08,0x2e,0xa8,0x20,0x08,0x2e,0xa8,0x20,0x08,0x2e,0xa8,0x20,0x08,0x2e,0xa8,0x20,0x08,0x2e,0xa8,0x20,0x08,0x2e,0xa8,0x20,0x08 ];

pub const LADDER_IMAGE: Image = Image {
    width: LADDER_WIDTH,
    height: LADDER_HEIGHT,
    flags: LADDER_FLAGS,
    data: &LADDER,
};


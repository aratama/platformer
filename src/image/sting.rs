// sting
use crate::wasm4::*;
use crate::image::Image;

const STING_WIDTH: u32 = 8;
const STING_HEIGHT: u32 = 8;
const STING_FLAGS: u32 = BLIT_1BPP;
const STING: [u8; 8] = [ 0xff,0xff,0xbd,0xdb,0xbd,0xff,0xc3,0xff ];

pub const STING_IMAGE: Image = Image {
    width: STING_WIDTH,
    height: STING_HEIGHT,
    flags: STING_FLAGS,
    data: &STING,
};


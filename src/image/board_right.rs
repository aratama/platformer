// board_right
use crate::wasm4::*;
use crate::image::Image;

const BOARD_RIGHT_WIDTH: u32 = 16;
const BOARD_RIGHT_HEIGHT: u32 = 16;
const BOARD_RIGHT_FLAGS: u32 = BLIT_2BPP;
const BOARD_RIGHT: [u8; 64] = [ 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x02,0x80,0x00,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xae,0xaa,0xaa,0xaa,0xaf,0xaa,0xaf,0xff,0xff,0xea,0xaf,0xff,0xff,0xfa,0xaf,0xff,0xff,0xea,0xaa,0xaa,0xaf,0xaa,0xaa,0xaa,0xae,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0xaa,0x00,0x03,0xc0,0x00,0x00,0x02,0x80,0x00 ];

pub const BOARD_RIGHT_IMAGE: Image = Image {
    width: BOARD_RIGHT_WIDTH,
    height: BOARD_RIGHT_HEIGHT,
    flags: BOARD_RIGHT_FLAGS,
    data: &BOARD_RIGHT,
};


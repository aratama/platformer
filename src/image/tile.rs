// tile
use crate::wasm4::*;
use crate::image::Image;

const TILE_WIDTH: u32 = 8;
const TILE_HEIGHT: u32 = 8;
const TILE_FLAGS: u32 = BLIT_2BPP;
const TILE: [u8; 16] = [ 0x55,0x55,0x55,0x56,0x55,0x56,0x55,0x56,0x55,0x56,0x55,0x56,0x55,0x56,0x6a,0xab ];

pub const TILE_IMAGE: Image = Image {
    width: TILE_WIDTH,
    height: TILE_HEIGHT,
    flags: TILE_FLAGS,
    data: &TILE,
};


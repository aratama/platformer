// tile
const TILE_WIDTH: u32 = 8;
const TILE_HEIGHT: u32 = 8;
pub const TILE_FLAGS: u32 = 1; // BLIT_2BPP
const TILE: [u8; 16] = [ 0x55,0x55,0x55,0x56,0x55,0x56,0x55,0x56,0x55,0x56,0x55,0x56,0x55,0x56,0x6a,0xab ];





use crate::image::Image;
pub static TILE_IMAGE: Image = Image {
    width: TILE_WIDTH,
    height: TILE_HEIGHT,
    frames: &[&TILE],
};
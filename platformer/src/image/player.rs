// player
const PLAYER_WIDTH: u32 = 8;
const PLAYER_HEIGHT: u32 = 16;
const PLAYER_FLAGS: u32 = 1; // BLIT_2BPP
const PLAYER: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x01, 0x45, 0x05, 0x15, 0x05, 0x14, 0x04, 0x10, 0x05, 0x50, 0x15, 0x54,
    0x17, 0x74, 0x16, 0x64, 0x15, 0x54, 0x3f, 0xfc, 0x3f, 0xfc, 0x1f, 0xf4, 0x05, 0x50, 0x04, 0x10,
];

use crate::image::Image;
pub static PLAYER_IMAGE: Image = Image {
    width: PLAYER_WIDTH,
    height: PLAYER_HEIGHT,
    frames: &[&PLAYER],
    flags: PLAYER_FLAGS
};

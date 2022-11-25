pub mod board_right;
pub mod board_up;
pub mod carrot;
pub mod climb;
pub mod jump;
pub mod ladder;
pub mod lie;
pub mod lookup;
pub mod player;
pub mod player_ladder0;
pub mod player_ladder1;
pub mod slip;
pub mod sting;
pub mod tile;
pub mod title;
pub mod walk0;
pub mod walk1;
pub mod walk2;
pub mod walk3;

use crate::wasm4;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: &'static [u8],
    pub flags: u32,
}

impl Image {
    pub fn draw(&self, x: i32, y: i32, flags: u32) {
        wasm4::blit(self.data, x, y, self.width, self.height, self.flags | flags);
    }
}

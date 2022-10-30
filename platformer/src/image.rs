pub mod bg;
pub mod board_right;
pub mod board_up;
pub mod climb;
pub mod fruit;
pub mod jump;
pub mod lie;
pub mod lookup;
pub mod player;
pub mod walk;

use crate::palette::set_draw_color;
use crate::wasm4;

#[derive(Clone, Copy)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub frames: &'static [&'static [u8]],
}

impl Image {
    pub fn draw(&self, x: i32, y: i32, flags: u32) {
        set_draw_color(0x4320);
        let frame = self.frames[0];
        wasm4::blit(&frame, x, y, self.width, self.height, flags);
    }

    pub fn animate(&self, x: i32, y: i32, flags: u32, frame_count: u32, speed: u32) {
        set_draw_color(0x4320);
        let frame = self.frames[((frame_count / speed) as usize % self.frames.len())];
        wasm4::blit(&frame, x, y, self.width, self.height, flags);
    }
}

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
    pub data: &'static [u8],
}

impl Image {
    pub fn draw(&self, x: i32, y: i32, flags: u32) {
        set_draw_color(0x4320);
        wasm4::blit(&self.data, x, y, self.width, self.height, flags);
    }
}

#[derive(Clone, Copy)]
pub struct Animation {
    pub frames: &'static [Image],
}

impl Animation {
    pub fn draw(&self, x: i32, y: i32, flags: u32, frame_count: u32, speed: u32) {
        self.frames[((frame_count / speed) as usize % self.frames.len())].draw(x, y, flags)
    }
}

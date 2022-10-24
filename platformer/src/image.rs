pub mod fruit;
pub mod lie;
pub mod lookup;
pub mod player;

use crate::palette::set_draw_color;
use crate::wasm4;

#[derive(Clone, Copy)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: &'static [u8],
}

impl Image {
    pub fn draw(self, x: i32, y: i32, flags: u32) {
        set_draw_color(0x4320);
        wasm4::blit(&self.data, x, y, self.width, self.height, flags);
    }
}

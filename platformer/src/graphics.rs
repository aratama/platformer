use crate::image::{Animation, Image};
use crate::wasm4;

#[derive(Clone, Copy, Default)]
pub struct Graphics {
    pub frame_count: u32,
    pub debug: bool,
    pub dx: i32,
    pub dy: i32,
}

impl Graphics {
    pub fn blit(&self, sprite: &[u8], x: i32, y: i32, width: u32, height: u32, flags: u32) {
        wasm4::blit(sprite, x + self.dx, y + self.dy, width, height, flags);
    }

    pub fn draw(&self, image: Image, x: i32, y: i32, flags: u32) {
        image.draw(x + self.dx, y + self.dy, flags);
    }

    pub fn animate(&self, animation: Animation, x: i32, y: i32, flags: u32, speed: u32) {
        animation.draw(x + self.dx, y + self.dy, flags, self.frame_count, speed);
    }

    pub fn rect(&self, x: i32, y: i32, width: u32, height: u32) {
        wasm4::rect(x + self.dx, y + self.dy, width, height)
    }

    pub fn set_draw_color(&self, idx: u16) {
        unsafe { *wasm4::DRAW_COLORS = idx }
    }

    pub fn set_palette(&self, palette: [u32; 4]) {
        unsafe {
            *wasm4::PALETTE = palette;
        }
    }
}

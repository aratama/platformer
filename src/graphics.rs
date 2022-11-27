use crate::animation::Animation;
use crate::image::Image;
use crate::wasm4::*;

#[derive(Clone, Copy, Default)]
pub struct Graphics {
    pub frame_count: u32,
    pub debug: bool,
    pub dx: i32,
    pub dy: i32,
}

impl Graphics {
    pub fn blit(&self, sprite: &[u8], x: i32, y: i32, width: u32, height: u32, flags: u32) {
        blit(sprite, x + self.dx, y + self.dy, width, height, flags);
    }

    pub fn draw(&self, image: &Image, x: i32, y: i32, flags: u32) {
        image.draw(x + self.dx, y + self.dy, flags);
    }

    pub fn draw_sub(
        &self,
        image: &Image,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        src_x: u32,
        src_y: u32,
        flags: u32,
    ) {
        image.draw_sub(x + self.dx, y + self.dy, w, h, src_x, src_y, flags);
    }

    pub fn animate(&self, animation: &Animation, x: i32, y: i32, flags: u32, speed: u32) {
        animation.animate(x + self.dx, y + self.dy, flags, self.frame_count, speed);
    }

    pub fn rect(&self, x: i32, y: i32, width: u32, height: u32) {
        rect(x + self.dx, y + self.dy, width, height)
    }

    pub fn set_draw_color(&self, idx: u16) {
        unsafe { *DRAW_COLORS = idx }
    }

    pub fn set_palette(&self, palette: [u32; 4]) {
        unsafe {
            *PALETTE = palette;
        }
    }
}

use crate::animation::Animation;
use crate::image::Image;
use crate::palette::set_draw_color;
use crate::wasm4::*;

#[derive(Clone, Copy, Default)]
pub struct Graphics {
    pub frame_count: u32,
    pub debug: bool,
    pub dx: i32,
    pub dy: i32,
}

impl Graphics {
    pub fn new(frame_count: u32) -> Graphics {
        Graphics {
            frame_count: frame_count,
            debug: false,
            dx: 0,
            dy: 0,
        }
    }

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

    pub fn draw_bold_text<T>(&self, str: T, x: i32, y: i32)
    where
        T: AsRef<[u8]>,
    {
        set_draw_color(0x04);
        text(&str, self.dx + x + 0, self.dy + y - 1);
        text(&str, self.dx + x + 0, self.dy + y + 1);
        text(&str, self.dx + x - 1, self.dy + y + 0);
        text(&str, self.dx + x + 1, self.dy + y + 0);
        text(&str, self.dx + x - 1, self.dy + y - 1);
        text(&str, self.dx + x + 1, self.dy + y - 1);
        text(&str, self.dx + x - 1, self.dy + y + 1);
        text(&str, self.dx + x + 1, self.dy + y + 1);
        set_draw_color(0x01);
        text(&str, self.dx + x + 0, self.dy + y + 0);
    }

    pub fn text<T>(&self, str: T, x: i32, y: i32)
    where
        T: AsRef<[u8]>,
    {
        text(&str, self.dx + x, self.dy + y)
    }

    pub fn transate(&mut self, dx: i32, dy: i32) {
        self.dx += dx;
        self.dy += dy;
    }
}

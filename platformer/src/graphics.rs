use crate::image::Image;
use crate::wasm4;

#[derive(Clone, Copy, Default)]
pub struct Graphics {
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

    pub fn rect(&self, x: i32, y: i32, width: u32, height: u32) {
        wasm4::rect(x + self.dx, y + self.dy, width, height)
    }
}

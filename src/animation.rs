pub mod walk;
use crate::image::Image;
use crate::wasm4::*;

pub struct Animation {
    frames: &'static [&'static Image],
}

impl Animation {
    pub fn animate(&self, x: i32, y: i32, flags: u32, frame_count: u32, speed: u32) {
        let frame = self.frames[((frame_count / speed) as usize % self.frames.len())];
        blit(
            frame.data,
            x,
            y,
            frame.width,
            frame.height,
            frame.flags | flags,
        );
    }
}

use crate::graphics::Graphics;
use crate::palette::set_draw_color;
use crate::world_map::{WORLD, WORLD_HEIGHT, WORLD_WIDTH};

pub struct World {}

impl World {
    pub fn new() -> World {
        World {}
    }

    pub fn get_cell(&self, x: i32, y: i32) -> u32 {
        if 0 <= x && x <= WORLD_WIDTH as i32 && 0 <= y && y < WORLD_HEIGHT as i32 {
            let i = (WORLD_WIDTH as i32 * y + x) as usize;
            let s = WORLD[i..(i + 1)].to_string();
            if s == "#" {
                return 1;
            } else {
                return 0;
            }
        } else {
            return 1;
        }
    }

    pub fn draw(&self, g: Graphics) {
        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
                let cell = self.get_cell(x as i32, y as i32);
                if cell != 0 {
                    set_draw_color(0x44);
                    g.rect(8 * x as i32, 8 * y as i32, 8, 8);
                }
            }
        }
    }
}

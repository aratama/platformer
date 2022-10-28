use crate::graphics::Graphics;
use crate::palette::set_draw_color;
use crate::world_map::{WORLD, WORLD_HEIGHT, WORLD_WIDTH};

pub const CELL_SIZE: u32 = 8;

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
        let min_x = u32::min(i32::max(0, -g.dx) as u32 / CELL_SIZE, WORLD_WIDTH);
        let max_x = u32::min(min_x + (160 / CELL_SIZE) + 1, WORLD_WIDTH);
        let min_y = u32::min(i32::max(0, -g.dy) as u32 / CELL_SIZE, WORLD_HEIGHT);
        let max_y = u32::min(min_y + (160 / CELL_SIZE) + 1, WORLD_HEIGHT);
        for y in min_y..(max_y + 1) {
            for x in min_x..max_x {
                let cell = self.get_cell(x as i32, y as i32);
                if cell != 0 {
                    set_draw_color(0x44);
                    g.rect(
                        (CELL_SIZE * x) as i32,
                        (CELL_SIZE * y) as i32,
                        CELL_SIZE,
                        CELL_SIZE,
                    );
                }
            }
        }
    }
}

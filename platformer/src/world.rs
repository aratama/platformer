use crate::graphics::Graphics;
use crate::image::board_right::{BOARD_RIGHT_FLAGS, BOARD_RIGHT_IMAGE};
use crate::image::board_up::{BOARD_UP_FLAGS, BOARD_UP_IMAGE};
use crate::palette::set_draw_color;
use crate::wasm4;
use crate::world_map::{WORLD, WORLD_HEIGHT, WORLD_WIDTH};
pub const CELL_SIZE: u32 = 8;

pub struct World {
    width: u32,
    height: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Block {
    Empty,
    Wall,
    RightArrow,
    UpArrow,
}

impl World {
    pub fn new() -> World {
        World {
            width: WORLD_WIDTH,
            height: WORLD.len() as u32 / WORLD_WIDTH,
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Block {
        if 0 <= x && x <= self.width as i32 && 0 <= y && y < self.height as i32 {
            let i = (WORLD_WIDTH as i32 * y + x) as usize;
            let s = WORLD[i..(i + 1)].to_string();
            if s == "#" {
                return Block::Wall;
            } else if s == "R" {
                return Block::RightArrow;
            } else if s == "U" {
                return Block::UpArrow;
            } else {
                return Block::Empty;
            }
        } else {
            return Block::Wall;
        }
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        if 0 <= x && x <= self.width as i32 && 0 <= y && y < self.height as i32 {
            let i = (WORLD_WIDTH as i32 * y + x) as usize;
            let s = WORLD[i..(i + 1)].to_string();
            if s == "#" {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    pub fn draw(&self, g: Graphics) {
        let min_x = u32::min(
            // 看板のように基準位置より右に描くものがあるので、 - CELL_SIZE で少し広めにとる
            i32::max(0, -g.dx - CELL_SIZE as i32) as u32 / CELL_SIZE,
            self.width as u32,
        );
        let max_x = u32::min(
            min_x + (wasm4::SCREEN_SIZE / CELL_SIZE) + 2,
            self.width as u32,
        );
        let min_y = u32::min(
            // 看板のように基準位置より右に描くものがあるので、 - CELL_SIZE で少し広めにとる
            i32::max(0, -g.dy - CELL_SIZE as i32) as u32 / CELL_SIZE,
            self.height as u32,
        );
        let max_y = u32::min(
            min_y + (wasm4::SCREEN_SIZE / CELL_SIZE) + 2,
            self.height as u32,
        );
        for y in min_y..(max_y + 1) {
            for x in min_x..max_x {
                let cell = self.get_cell(x as i32, y as i32);
                match cell {
                    Block::Empty => {}

                    Block::Wall => {
                        set_draw_color(0x44);
                        g.rect(
                            (CELL_SIZE * x) as i32,
                            (CELL_SIZE * y) as i32,
                            CELL_SIZE,
                            CELL_SIZE,
                        );
                    }

                    Block::RightArrow => {
                        set_draw_color(0x44);
                        g.draw(
                            BOARD_RIGHT_IMAGE,
                            (CELL_SIZE * x) as i32,
                            (CELL_SIZE * y) as i32,
                            BOARD_RIGHT_FLAGS,
                        );
                    }

                    Block::UpArrow => {
                        set_draw_color(0x44);
                        g.draw(
                            BOARD_UP_IMAGE,
                            (CELL_SIZE * x) as i32,
                            (CELL_SIZE * y) as i32,
                            BOARD_UP_FLAGS,
                        );
                    }
                }
            }
        }
    }
}

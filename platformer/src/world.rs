use crate::graphics::Graphics;
use crate::palette::set_draw_color;

const WORLD_WIDTH: u32 = 32;
const WORLD_HEIGHT: u32 = 32;
const WORLD: &str = r#################################"
###############################
##                            #
###                           #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
##          ###             ###
##                        #####
###              ##############
###          ##################
###############################
###############################
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
#                             #
###############################
"#################################;

const WORLD_SIZE: usize = 755; // 756 cause oom error :/

pub struct World {
    cells: [u8; WORLD_SIZE],
}

impl World {
    pub fn new() -> World {
        let mut cells = [0; WORLD_SIZE];

        for x in cells.iter_mut() {
            *x = 100;
        }

        World { cells: cells }
    }

    pub fn getCell(&self, x: i32, y: i32) -> u32 {
        if 0 <= x && x < WORLD_WIDTH as i32 && 0 <= y && y < WORLD_HEIGHT as i32 {
            let i = (WORLD_WIDTH as i32 * y + x) as usize;
            let s = WORLD[i..(i + 1)].to_string();
            if s == "#" {
                return 1;
            } else {
                return 0;
            }
        } else {
            return 100;
        }
    }

    pub fn draw(&self, g: Graphics) {
        for y in 0..WORLD_WIDTH {
            for x in 0..WORLD_HEIGHT {
                let cell = self.getCell(x as i32, y as i32);
                if cell != 0 {
                    set_draw_color(0x44);
                    g.rect(8 * x as i32, 8 * y as i32, 8, 8);
                }
            }
        }
    }
}

pub fn getC(x: i32, y: i32) -> String {
    if 0 <= x && x < WORLD_WIDTH as i32 && 0 <= y && y < WORLD_HEIGHT as i32 {
        let i = (WORLD_WIDTH as i32 * y + x) as usize;
        let s = WORLD[i..(i + 1)].to_string();
        return s;
    } else {
        return "OUT".to_string();
    }
}

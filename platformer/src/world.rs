use crate::palette::set_draw_color;
use crate::wasm4;

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
####                          #
####                          #
####                          #
####                          #
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
#                             #
###############################
"#################################;

pub fn getCell(x: u32, y: u32) -> u32 {
    if 0 <= x && x < WORLD_WIDTH && 0 <= y && y < WORLD_HEIGHT {
        let i = (WORLD_WIDTH * y + (x + 1)) as usize;
        let s = WORLD[i..(i + 1)].to_string();
        if s == "#" {
            return 1;
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}

pub fn draw() {
    // set_draw_color(0x40);
    // wasm4::rect(
    //     0, 0, 8, 8
    // );

    // set_draw_color(0x41);
    // wasm4::rect(
    //     10, 0, 8, 8
    // );

    // set_draw_color(0x42);
    // wasm4::rect(
    //     20, 0, 8, 8
    // );

    // set_draw_color(0x43);
    // wasm4::rect(
    //     30, 0, 8, 8
    // );

    // set_draw_color(0x44);
    // wasm4::rect(
    //     40, 0, 8, 8
    // );

    for y in 0..WORLD_WIDTH {
        for x in 0..WORLD_HEIGHT {
            let cell = getCell(x, y);
            if cell != 0 {
                set_draw_color(0x44);
                wasm4::rect(8 * x as i32, 8 * y as i32, 8, 8);
            }
        }
    }
}

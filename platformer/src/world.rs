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

pub fn getCell(x: i32, y: i32) -> u32 {
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

pub fn getC(x: i32, y: i32) -> String {
    if 0 <= x && x < WORLD_WIDTH as i32 && 0 <= y && y < WORLD_HEIGHT as i32 {
        let i = (WORLD_WIDTH as i32 * y + x) as usize;
        let s = WORLD[i..(i + 1)].to_string();
        return s;
    } else {
        return "OUT".to_string();
    }
}

pub fn draw(g: Graphics) {
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
            let cell = getCell(x as i32, y as i32);
            if cell != 0 {
                set_draw_color(0x44);
                g.rect(8 * x as i32, 8 * y as i32, 8, 8);
            }
        }
    }
}

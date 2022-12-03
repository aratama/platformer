use crate::geometry::direction::Direction;
use crate::wasm4::*;

#[derive(Clone, Copy)]
pub struct Inputs {
    gamepad: u8,
    prev_gamepad: u8,
}

static mut PREV_GAMEPADS: [u8; 4] = [0; 4];

impl Inputs {
    pub fn new(index: usize) -> Self {
        unsafe {
            let gamepad = match index {
                0 => *GAMEPAD1,
                1 => *GAMEPAD2,
                2 => *GAMEPAD3,
                3 => *GAMEPAD4,
                _ => 0,
            };
            Self {
                gamepad,
                prev_gamepad: PREV_GAMEPADS[index],
            }
        }
    }

    pub fn is_button_pressed(&self, button: u8) -> bool {
        (self.gamepad) & button != 0
    }

    pub fn is_button_just_pressed(&self, button: u8) -> bool {
        let just = self.gamepad & (self.gamepad ^ self.prev_gamepad);
        (just) & button != 0
    }

    pub fn is_any_button_just_pressed(&self) -> bool {
        self.is_button_just_pressed(BUTTON_1)
            || self.is_button_just_pressed(BUTTON_2)
            || self.is_button_just_pressed(BUTTON_UP)
            || self.is_button_just_pressed(BUTTON_DOWN)
            || self.is_button_just_pressed(BUTTON_LEFT)
            || self.is_button_just_pressed(BUTTON_RIGHT)
    }

    pub fn horizontal_acceralation(&self) -> f32 {
        let l = if self.is_button_pressed(BUTTON_LEFT) {
            1.0
        } else {
            0.0
        };
        let r = if self.is_button_pressed(BUTTON_RIGHT) {
            1.0
        } else {
            0.0
        };
        r - l
    }

    pub fn up_down(&self) -> i8 {
        let up = if self.is_button_pressed(BUTTON_UP) {
            1
        } else {
            0
        };
        let down = if self.is_button_pressed(BUTTON_DOWN) {
            1
        } else {
            0
        };
        down - up
    }

    /**
     * 左右矢印キーの状態をDirectionで返します
     */
    pub fn direction(&self) -> Option<Direction> {
        if self.is_button_pressed(BUTTON_LEFT) {
            Some(Direction::Left)
        } else if self.is_button_pressed(BUTTON_RIGHT) {
            Some(Direction::Right)
        } else {
            None
        }
    }
}

pub fn update_gamepads() {
    unsafe {
        PREV_GAMEPADS[0] = *GAMEPAD1;
        PREV_GAMEPADS[1] = *GAMEPAD2;
        PREV_GAMEPADS[2] = *GAMEPAD3;
        PREV_GAMEPADS[3] = *GAMEPAD4;
    }
}

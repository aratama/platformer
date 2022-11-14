use crate::geometry::direction::Direction;
use crate::wasm4;

#[derive(Clone, Copy)]
pub struct Inputs {
    gamepad: u8,
    prev_gamepad: u8,
}

impl Inputs {
    pub fn new(gamepad: u8, prev_gamepad: u8) -> Self {
        Self {
            gamepad,
            prev_gamepad,
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
        self.is_button_just_pressed(wasm4::BUTTON_1)
            || self.is_button_just_pressed(wasm4::BUTTON_2)
            || self.is_button_just_pressed(wasm4::BUTTON_UP)
            || self.is_button_just_pressed(wasm4::BUTTON_DOWN)
            || self.is_button_just_pressed(wasm4::BUTTON_LEFT)
            || self.is_button_just_pressed(wasm4::BUTTON_RIGHT)
    }

    pub fn horizontal_acceralation(&self) -> f32 {
        let l = if self.is_button_pressed(wasm4::BUTTON_LEFT) {
            1.0
        } else {
            0.0
        };
        let r = if self.is_button_pressed(wasm4::BUTTON_RIGHT) {
            1.0
        } else {
            0.0
        };
        r - l
    }

    /**
     * 左右矢印キーの状態をDirectionで返します
     */
    pub fn direction(&self) -> Option<Direction> {
        if self.is_button_pressed(wasm4::BUTTON_LEFT) {
            Some(Direction::Left)
        } else if self.is_button_pressed(wasm4::BUTTON_RIGHT) {
            Some(Direction::Right)
        } else {
            None
        }
    }
}

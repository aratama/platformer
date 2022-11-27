use crate::wasm4::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> i32 {
        if *self == Direction::Left {
            -1
        } else {
            1
        }
    }

    pub fn from_delta(delta: f32, def: Direction) -> Direction {
        if delta == 0.0 {
            def
        } else if 0.0 <= delta {
            Direction::Right
        } else {
            Direction::Left
        }
    }

    pub fn to_flags(&self) -> u32 {
        if *self == Direction::Right {
            0
        } else {
            BLIT_FLIP_X
        }
    }
}

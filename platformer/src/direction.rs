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

    pub fn fromDelta(delta: f32, def: Direction) -> Direction {
        if delta == 0.0 {
            def
        } else if 0.0 <= delta {
            Direction::Right
        } else {
            Direction::Left
        }
    }
}

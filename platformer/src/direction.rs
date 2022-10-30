#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
}

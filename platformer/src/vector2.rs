use std::ops;

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, q: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + q.x,
            y: self.y + q.y,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, q: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - q.x,
            y: self.y - q.y,
        }
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, t: f32) -> Vector2 {
        Vector2 {
            x: self.x * t,
            y: self.y * t,
        }
    }
}

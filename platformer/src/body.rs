use crate::image::Image;
use crate::vector2::Vector2;
use crate::wasm4;

const MAX_VELOCITY: f32 = 10.0;
const MAX_PLAYER_Y: f32 = 100.0;
const JUMP_VELOCITY: f32 = 2.5;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right,
}

pub struct Body<'a> {
    pub position: Vector2,
    pub velocity: Vector2,
    pub image: Image<'a>,
    pub direction: Direction,
}

impl<'a> Body<'a> {
    pub fn new(position: Vector2, image: Image<'a>) -> Self {
        Self {
            position,
            velocity: Vector2::default(),
            image,
            direction: Direction::Right,
        }
    }

    pub fn draw(&self) {
        self.image.draw(
            self.position.x.floor() as i32,
            self.position.y.floor() as i32,
            wasm4::BLIT_2BPP
                | if self.direction == Direction::Right {
                    0
                } else {
                    wasm4::BLIT_FLIP_X
                },
        );
    }

    pub fn update(&mut self) {
        let gravity: Vector2 = Vector2::new(0.0, 0.1);

        self.velocity.y = f32::max(
            -MAX_VELOCITY,
            f32::min(MAX_VELOCITY, self.velocity.y + gravity.y),
        );
        self.velocity.x = f32::max(
            -MAX_VELOCITY,
            f32::min(MAX_VELOCITY, self.velocity.x + gravity.x),
        );

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        self.position.y = f32::min(MAX_PLAYER_Y, self.position.y)
    }

    pub fn walk(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn left(&mut self) {
        self.direction = Direction::Left;
        self.walk(-1.0, 0.0)
    }

    pub fn right(&mut self) {
        self.direction = Direction::Right;
        self.walk(1.0, 0.0)
    }

    pub fn up(&mut self) {
        self.walk(0.0, -1.0)
    }

    pub fn down(&mut self) {
        self.walk(0.0, 1.0)
    }

    pub fn jump(&mut self) {
        self.velocity.y = -JUMP_VELOCITY
    }
}

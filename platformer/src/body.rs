use crate::image::Image;
use crate::input::Inputs;
use crate::lie::LIE_IMAGE;
use crate::lookup::LOOKUP_IMAGE;
use crate::vector2::Vector2;
use crate::wasm4;
use crate::Game;

const MAX_VELOCITY: f32 = 10.0;
const MAX_PLAYER_Y: f32 = 100.0;
const JUMP_VELOCITY: f32 = 2.5;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right,
}

pub enum Pose {
    Stand,
    Lie,
    LookUp,
}

pub struct Body<'a> {
    pub position: Vector2,
    pub velocity: Vector2,
    pub image: Image<'a>,
    pub direction: Direction,
    pub pose: Pose,
}

impl<'a> Body<'a> {
    pub fn new(position: Vector2, image: Image<'a>) -> Self {
        Self {
            position,
            velocity: Vector2::default(),
            image,
            direction: Direction::Right,
            pose: Pose::Stand,
        }
    }

    pub fn draw(&self) {
        let flags = wasm4::BLIT_2BPP
            | if self.direction == Direction::Right {
                0
            } else {
                wasm4::BLIT_FLIP_X
            };

        match self.pose {
            Pose::Stand => {
                self.image.draw(
                    self.position.x.floor() as i32,
                    self.position.y.floor() as i32,
                    flags,
                );
            }

            Pose::Lie => {
                LIE_IMAGE.draw(
                    self.position.x.floor() as i32,
                    self.position.y.floor() as i32,
                    flags,
                );
            }
            Pose::LookUp => {
                LOOKUP_IMAGE.draw(
                    self.position.x.floor() as i32,
                    self.position.y.floor() as i32,
                    flags,
                );
            }
        }
    }

    pub fn update(&mut self, inputs: Inputs) {
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

        self.position.y = f32::min(MAX_PLAYER_Y, self.position.y);

        self.pose = if inputs.is_button_pressed(wasm4::BUTTON_DOWN) {
            Pose::Lie
        } else if inputs.is_button_pressed(wasm4::BUTTON_UP) {
            Pose::LookUp
        } else {
            Pose::Stand
        }
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
        self.pose = Pose::Lie;
        self.velocity.y = -JUMP_VELOCITY
    }
}

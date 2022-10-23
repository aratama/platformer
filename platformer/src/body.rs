use crate::image::Image;
use crate::input::Inputs;
use crate::lie::LIE_IMAGE;
use crate::player::PLAYER_IMAGE;

use crate::lookup::LOOKUP_IMAGE;
use crate::vector2::Vector2;
use crate::wasm4;
use crate::world;
use crate::Game;

const MAX_VELOCITY: f32 = 100.0;
const JUMP_VELOCITY: f32 = 2.5;
const WALK_VELOCITY: f32 = 2.0;

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
    pub name: &'a str,
    pub position: Vector2,
    pub velocity: Vector2,
    pub image: Image<'a>,
    pub direction: Direction,
    pub pose: Pose,
}

impl<'a> Body<'a> {
    pub fn new(name: &'a str, position: Vector2, image: Image<'a>) -> Self {
        Self {
            name,
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

        let i: Image = match self.pose {
            Pose::Stand => self.image,
            Pose::Lie => LIE_IMAGE,
            Pose::LookUp => LOOKUP_IMAGE,
        };
        let x = (i.width as f32 * -0.5 + self.position.x.floor()) as i32;
        let y = (-1.0 * i.height as f32 + 1.0 + self.position.y.floor()) as i32;

        i.draw(x, y, flags);
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

        self.delta(self.velocity.x, 0.0);
        self.delta(0.0, self.velocity.y);

        self.pose = if inputs.is_button_pressed(wasm4::BUTTON_DOWN) {
            Pose::Lie
        } else if inputs.is_button_pressed(wasm4::BUTTON_UP) {
            Pose::LookUp
        } else {
            Pose::Stand
        }
    }

    pub fn delta(&mut self, vx: f32, vy: f32) {
        if f32::abs(vy) <= f32::abs(vx) {
            let mut dx = vx;
            let mut dy = vy;
            while 1.0 <= f32::abs(dx) {
                let sign = if 0.0 < dx { 1.0 } else { -1.0 };
                let px = self.position.x + sign;
                let py = self.position.y + sign * (vy / vx);

                let cx = (px / 8.0).floor() as u32;
                let cy = (py / 8.0).floor() as u32;
                let cell = world::getCell(cx, cy);

                if cell == 0 {
                    dx -= sign;
                    dy -= sign;
                    self.position.x = px;
                    self.position.y = py;
                } else {
                    self.velocity.x = 0.0;
                    self.velocity.y = 0.0;
                    break;
                }
            }
        } else {
            let mut dx = vx;
            let mut dy = vy;
            while 1.0 <= f32::abs(dy) {
                let sign = if 0.0 < dy { 1.0 } else { -1.0 };
                let px = self.position.x + sign * (vx / vy);
                let py = self.position.y + sign;

                let cx = (px / 8.0).floor() as u32;
                let cy = (py / 8.0).floor() as u32;
                let cell = world::getCell(cx, cy);

                if cell == 0 {
                    dx -= sign;
                    dy -= sign;
                    self.position.x = px;
                    self.position.y = py;
                } else {
                    self.velocity.x = 0.0;
                    self.velocity.y = 0.0;
                    break;
                }
            }

            let px = self.position.x + dx;
            let py = self.position.y + dy;
            let cx = (px / 8.0).floor() as u32;
            let cy = (py / 8.0).floor() as u32;
            let cell = world::getCell(cx, cy);
            if cell == 0 {
                // self.position.x = px;
                self.position.y = py;
            }
        }
    }

    pub fn walk(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn left(&mut self) {
        self.direction = Direction::Left;
        self.velocity.x = -WALK_VELOCITY;
    }

    pub fn right(&mut self) {
        self.direction = Direction::Right;
        self.velocity.x = WALK_VELOCITY;
    }

    pub fn jump(&mut self) {
        self.pose = Pose::Lie;
        self.velocity.y = -JUMP_VELOCITY
    }
}

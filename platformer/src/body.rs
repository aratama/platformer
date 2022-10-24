use crate::image::lie::LIE_IMAGE;
use crate::image::Image;
use crate::input::Inputs;

use crate::image::lookup::LOOKUP_IMAGE;
use crate::vector2::Vector2;
use crate::wasm4;
use crate::world;

const MAX_VELOCITY: f32 = 100.0;
const JUMP_VELOCITY: f32 = 2.5;
const WALK_VELOCITY: f32 = 1.0;
const CRAWL_VELOCITY: f32 = 0.25;

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

pub struct Body {
    pub name: &'static str,
    pub position: Vector2,
    pub velocity: Vector2,
    pub image: Image,
    pub direction: Direction,
    pub pose: Pose,
}

impl Body {
    pub fn new(name: &'static str, position: Vector2, image: Image) -> Self {
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

        self.delta(self.velocity.x, self.velocity.y);

        self.pose = if inputs.is_button_pressed(wasm4::BUTTON_DOWN) {
            Pose::Lie
        } else if inputs.is_button_pressed(wasm4::BUTTON_UP) {
            Pose::LookUp
        } else {
            Pose::Stand
        }
    }

    pub fn delta(&mut self, vx: f32, vy: f32) {
        let mut dx = vx;
        while 0.001 <= f32::abs(dx) {
            let sign = if 0.0 < dx { 1.0 } else { -1.0 };
            let stride = sign * f32::min(1.0, f32::abs(dx));
            let px = self.position.x + stride;
            let cx = (px / 8.0).floor() as i32;
            let cy = (self.position.y.floor() / 8.0) as i32;
            let cell = world::getCell(cx, cy);
            if cell == 0 {
                dx -= stride;
                self.position.x = px;
            } else {
                self.velocity.x = 0.0;
                break;
            }
        }

        let mut dy = vy;
        while 0.001 <= f32::abs(dy) {
            let sign = if 0.0 < dy { 1.0 } else { -1.0 };
            let stride = sign * f32::min(1.0, f32::abs(dy));
            let py = self.position.y + stride;
            let cx = (self.position.x.floor() / 8.0) as i32;
            let cy = (py / 8.0).floor() as i32;
            let cell = world::getCell(cx, cy);
            if cell == 0 {
                dy -= stride;
                self.position.y = py;
            } else {
                self.velocity.y = 0.0;
                break;
            }
        }
    }

    pub fn jump(&mut self) {
        self.pose = Pose::Lie;
        self.velocity.y = -JUMP_VELOCITY
    }

    pub fn walk(&mut self, speed: f32, input: Inputs) {
        self.velocity.x = WALK_VELOCITY
            * speed
            * if input.is_button_pressed(wasm4::BUTTON_DOWN) {
                CRAWL_VELOCITY
            } else {
                1.0
            };
    }

    pub fn input(&mut self, input: Inputs) {
        if input.is_button_pressed(wasm4::BUTTON_LEFT) {
            self.direction = Direction::Left;
            self.walk(-1.0, input);
        }
        if input.is_button_pressed(wasm4::BUTTON_RIGHT) {
            self.direction = Direction::Right;
            self.walk(1.0, input);
        }

        if !input.is_button_pressed(wasm4::BUTTON_LEFT)
            && !input.is_button_pressed(wasm4::BUTTON_RIGHT)
        {
            self.velocity.x = 0.0;
        }

        if input.is_button_just_pressed(wasm4::BUTTON_1) {
            self.jump()
        }
    }
}

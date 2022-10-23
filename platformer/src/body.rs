use crate::image::Image;
use crate::vector2::Vector2;
use crate::{palette::set_draw_color, wasm4};
use std::cmp;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

const FRUIT_SPRITE: Image = [
    0x00, 0xa0, 0x02, 0x00, 0x0e, 0xf0, 0x36, 0x5c, 0xd6, 0x57, 0xd5, 0x57, 0x35, 0x5c, 0x0f, 0xf0,
];

const MAX_VELOCITY: f32 = 10.0;

pub struct Body {
    pub position: Vector2,
    pub velocity: Vector2,
}

impl Body {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            velocity: Vector2::default(),
        }
    }

    pub fn draw(&self) {
        set_draw_color(0x4320);
        wasm4::blit(
            &FRUIT_SPRITE,
            self.position.x.floor() as i32,
            self.position.y.floor() as i32,
            8,
            8,
            wasm4::BLIT_2BPP,
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

        self.position.y = f32::min(100.0, self.position.y)
    }

    pub fn walk(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn left(&mut self) {
        self.walk(-1.0, 0.0)
    }

    pub fn right(&mut self) {
        self.walk(1.0, 0.0)
    }

    pub fn up(&mut self) {
        self.walk(0.0, -1.0)
    }

    pub fn down(&mut self) {
        self.walk(0.0, 1.0)
    }

    pub fn jump(&mut self) {
        self.velocity.y = -2.5
    }
}

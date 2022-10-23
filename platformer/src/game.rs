use crate::body::{Body, Point};
use crate::image::Image;
use crate::palette::set_draw_color;
use crate::vector2::Vector2;
use crate::wasm4;
use fastrand::Rng;

const FRUIT_SPRITE: Image = [
    0x00, 0xa0, 0x02, 0x00, 0x0e, 0xf0, 0x36, 0x5c, 0xd6, 0x57, 0xd5, 0x57, 0x35, 0x5c, 0x0f, 0xf0,
];

pub struct Game {
    rng: Rng,
    frame_count: u32,
    player: Body,
    prev_gamepad: u8,
    fruit: Body,
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);

        Self {
            frame_count: 0,
            player: Body::new(Vector2::new(8.0 * 4.0, 8.0 * 4.0)),
            prev_gamepad: 0,
            fruit: Body::new(Vector2::new(
                rng.i32(0..wasm4::SCREEN_SIZE as i32) as f32,
                rng.i32(0..wasm4::SCREEN_SIZE as i32) as f32,
            )),
            rng,
        }
    }

    pub fn isButtonPressed(&mut self, button: u8) -> bool {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just = gamepad & (gamepad ^ self.prev_gamepad);
        (gamepad) & button != 0
    }

    pub fn isButtonJustPressed(&mut self, button: u8) -> bool {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just = gamepad & (gamepad ^ self.prev_gamepad);
        (just) & button != 0
    }

    pub fn input(&mut self) {
        if self.isButtonPressed(wasm4::BUTTON_LEFT) {
            self.player.left();
        }
        if self.isButtonPressed(wasm4::BUTTON_RIGHT) {
            self.player.right();
        }
        if self.isButtonPressed(wasm4::BUTTON_UP) {
            self.player.up();
        }
        if self.isButtonPressed(wasm4::BUTTON_DOWN) {
            self.player.down();
        }

        if self.isButtonJustPressed(wasm4::BUTTON_1) {
            self.player.jump()
        }

        self.prev_gamepad = unsafe { *wasm4::GAMEPAD1 };
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        self.input();

        self.player.update();

        self.fruit.update();

        self.player.draw();

        self.fruit.draw();
    }
}

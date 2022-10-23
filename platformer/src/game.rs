use crate::body::Body;
use crate::fruit::FRUIT_IMAGE;
use crate::input::Inputs;
use crate::player::PLAYER_IMAGE;
use crate::vector2::Vector2;
use crate::wasm4;
use crate::world;
use fastrand::Rng;

pub struct Game<'a> {
    rng: Rng,
    frame_count: u32,
    player: Body<'a>,
    prev_gamepad: u8,
    fruit: Body<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);

        Self {
            frame_count: 0,
            player: Body::new(Vector2::new(8.0 * 4.0, 8.0 * 4.0), PLAYER_IMAGE),
            prev_gamepad: 0,
            fruit: Body::new(
                Vector2::new(
                    rng.i32(0..wasm4::SCREEN_SIZE as i32) as f32,
                    rng.i32(0..wasm4::SCREEN_SIZE as i32) as f32,
                ),
                FRUIT_IMAGE,
            ),
            rng,
        }
    }

    pub fn is_button_pressed(&mut self, button: u8) -> bool {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        (gamepad) & button != 0
    }

    pub fn is_button_just_pressed(&mut self, button: u8) -> bool {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just = gamepad & (gamepad ^ self.prev_gamepad);
        (just) & button != 0
    }

    pub fn input(&mut self) {
        if self.is_button_pressed(wasm4::BUTTON_LEFT) {
            self.player.left();
        }
        if self.is_button_pressed(wasm4::BUTTON_RIGHT) {
            self.player.right();
        }
        if self.is_button_pressed(wasm4::BUTTON_UP) {
            self.player.up();
        }
        if self.is_button_pressed(wasm4::BUTTON_DOWN) {
            self.player.down();
        }

        if self.is_button_just_pressed(wasm4::BUTTON_1) {
            self.player.jump()
        }

        self.prev_gamepad = unsafe { *wasm4::GAMEPAD1 };
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };

        self.frame_count += 1;

        self.input();

        self.player.update(Inputs::new(gamepad, self.prev_gamepad));

        self.fruit.update(Inputs::new(0, 0));

        world::draw();

        self.player.draw();

        self.fruit.draw();
    }
}

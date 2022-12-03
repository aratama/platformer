use crate::input::{update_gamepads, Inputs};

use crate::scene::title_scene::TitleScene;
use crate::scene::Scene;
use crate::sound::update_bgm;
use crate::wasm4::*;

pub struct Game {
    prev_gamepad1: u8,
    prev_gamepad2: u8,
    prev_gamepad3: u8,
    prev_gamepad4: u8,
    scene: Scene,
    player_active: [bool; 4],
}

impl Game {
    pub fn new() -> Self {
        Game {
            prev_gamepad1: 0,
            prev_gamepad2: 0,
            prev_gamepad3: 0,
            prev_gamepad4: 0,
            scene: Scene::TitleScene(TitleScene::new()),
            player_active: [true, false, false, false],
        }
    }

    pub fn update(&mut self) {
        let gamepad2 = unsafe { *GAMEPAD2 };
        let gamepad3 = unsafe { *GAMEPAD3 };
        let gamepad4 = unsafe { *GAMEPAD4 };

        // trace(format!("{}", gamepad2));

        self.player_active[1] |= 0 < gamepad2;
        self.player_active[2] |= 0 < gamepad3;
        self.player_active[3] |= 0 < gamepad4;

        let inputs = Inputs::new(0);
        let result = match { &mut self.scene } {
            Scene::TitleScene(t) => t.update(&inputs, &self.player_active),
            Scene::GameScene(g) => g.update(&inputs, &self.player_active),
            Scene::EndingScene(e) => e.update(&inputs, &self.player_active),
        };
        match result {
            Option::None => {}
            Option::Some(next) => self.scene = next,
        }
        self.prev_gamepad1 = unsafe { *GAMEPAD1 };

        update_bgm();
        update_gamepads();
    }
}

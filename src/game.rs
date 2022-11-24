use crate::input::Inputs;

use crate::scene::title_scene::TitleScene;
use crate::scene::Scene;
use crate::sound::update_bgm;
use crate::wasm4;

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
        let gamepad1 = unsafe { *wasm4::GAMEPAD1 };
        let gamepad2 = unsafe { *wasm4::GAMEPAD2 };
        let gamepad3 = unsafe { *wasm4::GAMEPAD3 };
        let gamepad4 = unsafe { *wasm4::GAMEPAD4 };

        self.player_active[1] |= 0 < gamepad2;
        self.player_active[2] |= 0 < gamepad3;
        self.player_active[3] |= 0 < gamepad4;

        let inputs = Inputs::new(gamepad1, self.prev_gamepad1);
        let result = match { &mut self.scene } {
            Scene::TitleScene(t) => t.update(&inputs, &self.player_active),
            Scene::GameScene(g) => g.update(&inputs),
            Scene::EndingScene(e) => e.update(&inputs),
        };
        match result {
            Option::None => {}
            Option::Some(next) => self.scene = next,
        }
        self.prev_gamepad1 = unsafe { *wasm4::GAMEPAD1 };

        update_bgm();
    }
}

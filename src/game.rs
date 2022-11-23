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

    player2_active: bool,
    player3_active: bool,
    player4_active: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            prev_gamepad1: 0,
            prev_gamepad2: 0,
            prev_gamepad3: 0,
            prev_gamepad4: 0,
            scene: Scene::TitleScene(TitleScene::new()),
            player2_active: false,
            player3_active: false,
            player4_active: false,
        }
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let inputs = Inputs::new(gamepad, self.prev_gamepad1);
        let result = match { &mut self.scene } {
            Scene::TitleScene(t) => t.update(&inputs),
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

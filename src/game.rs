use crate::input::Inputs;

use crate::scene::game_scene::GameScene;
use crate::scene::title_scene::TitleScene;
use crate::scene::Scene;
use crate::sound::update_bgm;
use crate::wasm4;

pub struct Game {
    prev_gamepad: u8,
    scene: Scene,
}

impl Game {
    pub fn new() -> Self {
        Game {
            prev_gamepad: 0,
            scene: Scene::TitleScene(TitleScene::new()),
        }
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let inputs = Inputs::new(gamepad, self.prev_gamepad);
        let result = match { &mut self.scene } {
            Scene::TitleScene(t) => t.update(&inputs),
            Scene::GameScene(g) => g.update(&inputs),
            Scene::EndingScene(e) => e.update(&inputs),
        };
        match result {
            Option::None => {}
            Option::Some(next) => self.scene = next,
        }
        self.prev_gamepad = unsafe { *wasm4::GAMEPAD1 };

        update_bgm();
    }
}

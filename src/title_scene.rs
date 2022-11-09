use crate::game::Scene;
use crate::input::Inputs;
use crate::wasm4::*;
pub struct TitleScene {
    prev_gamepad: u8,
}

impl TitleScene {
    pub fn new() -> Self {
        TitleScene { prev_gamepad: 0 }
    }

    pub fn update(&mut self) -> Scene {
        let gamepad = unsafe { *GAMEPAD1 };
        let inputs = Inputs::new(gamepad, self.prev_gamepad);

        text("TOWER CLIMBER", 30, 60);

        text("PRESS ANY KEY", 30, 110);
        text("TO START", 50, 120);

        self.prev_gamepad = unsafe { *GAMEPAD1 };

        if inputs.is_any_button_just_pressed() {
            Scene::GameScene
        } else {
            Scene::TitleScene
        }
    }
}

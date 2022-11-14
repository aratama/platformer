use crate::input::Inputs;
use crate::scene::Scene;
use crate::wasm4::*;

use crate::palette::set_draw_color;

pub struct EndingScene {
    prev_gamepad: u8,
}

impl EndingScene {
    pub fn new() -> Self {
        EndingScene { prev_gamepad: 0 }
    }

    pub fn update(&mut self) -> Scene {
        let gamepad = unsafe { *GAMEPAD1 };
        let inputs = Inputs::new(gamepad, self.prev_gamepad);

        set_draw_color(0x34);
        text("You Win", 10, 10);

        self.prev_gamepad = unsafe { *GAMEPAD1 };

        if inputs.is_any_button_just_pressed() {
            Scene::TitleScene
        } else {
            Scene::EndingScene
        }
    }
}

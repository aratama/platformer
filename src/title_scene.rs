use crate::game::Scene;
use crate::input::Inputs;
use crate::wasm4::*;

use crate::image::title::TITLE_IMAGE;
use crate::palette::set_draw_color;

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

        self.draw_title_image();

        self.prev_gamepad = unsafe { *GAMEPAD1 };

        if inputs.is_any_button_just_pressed() {
            Scene::GameScene
        } else {
            Scene::TitleScene
        }
    }

    fn draw_title_image(&self) {
        set_draw_color(0x4321);
        blit(
            TITLE_IMAGE.data,
            0,
            0,
            TITLE_IMAGE.width,
            TITLE_IMAGE.height,
            TITLE_IMAGE.flags,
        );
    }
}

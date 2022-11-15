use crate::input::Inputs;
use crate::scene::Scene;
use crate::wasm4::*;

use crate::image::title::TITLE_IMAGE;
use crate::palette::set_draw_color;

use super::game_scene::GameScene;

#[derive(Clone, Copy)]
pub struct TitleScene {
    prev_gamepad: u8,
}

impl TitleScene {
    pub fn new() -> Self {
        TitleScene { prev_gamepad: 0 }
    }

    pub fn update(&mut self, inputs: &Inputs) -> Option<Scene> {
        self.draw_title_image();

        self.prev_gamepad = unsafe { *GAMEPAD1 };

        if inputs.is_any_button_just_pressed() {
            Option::Some(Scene::GameScene(GameScene::new()))
        } else {
            Option::None
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

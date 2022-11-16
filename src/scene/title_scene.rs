use crate::input::Inputs;
use crate::scene::Scene;
use crate::sound::{set_bgm, TITLE_BGM_SCORE};
use crate::wasm4::*;

use crate::image::title::TITLE_IMAGE;
use crate::palette::set_draw_color;

use super::game_scene::GameScene;

#[derive(Clone, Copy)]
pub struct TitleScene {
    music_position: u32,
}

impl TitleScene {
    pub fn new() -> Self {
        TitleScene { music_position: 0 }
    }

    pub fn update(&mut self, inputs: &Inputs) -> Option<Scene> {
        self.draw_title_image();

        if inputs.is_any_button_just_pressed() {
            // set_bgm(Option::Some(TITLE_BGM_SCORE));
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

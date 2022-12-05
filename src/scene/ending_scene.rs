use crate::input::Inputs;
use crate::music::goal::GOAL_BGM_SCORE;
use crate::scene::Scene;
use crate::sound::set_bgm;
use crate::wasm4::*;

use crate::palette::set_draw_color;

use super::title_scene::TitleScene;

#[derive(Clone, Copy)]
pub struct EndingScene {}

impl EndingScene {
    pub fn new() -> Self {
        EndingScene {}
    }

    pub fn update(&mut self, inputs: &Inputs, _player_active: &[bool; 4]) -> Option<Scene> {
        set_draw_color(0x34);
        text("You Win!", 10, 10);

        set_bgm(Some(&GOAL_BGM_SCORE), false);

        if inputs.is_any_button_just_pressed() {
            Option::Some(Scene::TitleScene(TitleScene::new()))
        } else {
            Option::None
        }
    }
}

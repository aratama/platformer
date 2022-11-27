use crate::input::Inputs;
use crate::scene::Scene;
use crate::sound::set_bgm;
use crate::wasm4::*;

use crate::image::title::TITLE_IMAGE;
use crate::palette::set_draw_color;

use super::game_scene::GameScene;
use crate::music::level::LEVEL_BGM_SCORE;

#[derive(Clone, Copy)]
pub struct TitleScene {
    frame_count: u32,
}

impl TitleScene {
    pub fn new() -> Self {
        TitleScene { frame_count: 0 }
    }

    pub fn update(&mut self, inputs: &Inputs, player_active: &[bool; 4]) -> Option<Scene> {
        self.draw_title_image();

        set_bgm(Some(&LEVEL_BGM_SCORE));

        self.frame_count += 1;

        if inputs.is_any_button_just_pressed() {
            Option::Some(Scene::GameScene(GameScene::new(player_active)))
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

        if (self.frame_count / 48) % 2 == 0 {
            draw_bold_text(b"Press \x80 to Start", 15, 140);
        }
    }
}

fn draw_bold_text<T>(str: T, x: i32, y: i32)
where
    T: AsRef<[u8]>,
{
    set_draw_color(0x04);
    text(&str, x + 0, y - 1);
    text(&str, x + 0, y + 1);
    text(&str, x - 1, y + 0);
    text(&str, x + 1, y + 0);
    text(&str, x - 1, y - 1);
    text(&str, x + 1, y - 1);
    text(&str, x - 1, y + 1);
    text(&str, x + 1, y + 1);
    set_draw_color(0x01);
    text(&str, 15, 140);
}

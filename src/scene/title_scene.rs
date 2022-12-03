use crate::geometry::vector2::Vector2;
use crate::graphics::Graphics;
use crate::input::Inputs;
use crate::netplay::is_netplay_active;
use crate::save::{load, GameData};
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
    menu_visible: bool,
    menu_index: i8,
    save_data: Option<GameData>,
}

impl Graphics {
    fn draw_list(&mut self, items: &[&str], index: u8, x: i32, y: i32) {
        self.transate(x, y);
        set_draw_color(0x44);
        self.rect(2, 2, 80, 21);
        set_draw_color(0x41);
        self.rect(0, 0, 80, 21);
        set_draw_color(0x4);
        self.text("New Game", 12, 2);
        self.text("Continue", 12, 12);

        self.text(">", 4, 2 + 8 * index as i32);
        self.transate(-x, -y);
    }
}

impl TitleScene {
    pub fn new() -> Self {
        TitleScene {
            frame_count: 0,
            menu_visible: false,
            menu_index: 1,
            save_data: load(),
        }
    }

    pub fn update(&mut self, inputs: &Inputs, player_active: &[bool; 4]) -> Option<Scene> {
        set_draw_color(0x4321);
        blit(
            TITLE_IMAGE.data,
            0,
            0,
            TITLE_IMAGE.width,
            TITLE_IMAGE.height,
            TITLE_IMAGE.flags,
        );

        let mut g = Graphics::new(self.frame_count);

        if self.menu_visible {
            let items = [">New Game", " Continue"];
            g.draw_list(&items, self.menu_index as u8, 40, 120);
        } else {
            if (self.frame_count / 48) % 2 == 0 {
                draw_bold_text(b"Press \x80 to Start", 15, 140);
            }
        }

        set_draw_color(0x23);

        if is_netplay_active() {
            let g = Graphics::new(self.frame_count);
            g.draw_bold_text("Netplay Players:", 2, 2);
            let players: u32 = player_active
                .map(|active| if active { 1 } else { 0 })
                .iter()
                .sum();
            g.draw_bold_text(players.to_string(), 130, 2);
        }

        set_bgm(Some(&LEVEL_BGM_SCORE));

        self.frame_count += 1;

        if self.menu_visible {
            self.menu_index = i8::max(0, i8::min(1, self.menu_index + inputs.up_down()));

            if inputs.is_button_just_pressed(BUTTON_1) {
                return match self.menu_index {
                    0 if inputs.is_button_just_pressed(BUTTON_1) => Option::Some(Scene::GameScene(
                        GameScene::new(player_active, Option::None),
                    )),
                    1 if inputs.is_button_just_pressed(BUTTON_1) => match load() {
                        Some(data) => {
                            let start_position = Option::Some(Vector2::new(data.x, data.y));
                            Option::Some(Scene::GameScene(GameScene::new(
                                player_active,
                                start_position,
                            )))
                        }
                        None => Option::None,
                    },
                    _ => Option::None,
                };
            }
        } else {
            if inputs.is_button_just_pressed(BUTTON_1) {
                return match self.save_data {
                    Option::None => Option::Some(Scene::GameScene(GameScene::new(
                        player_active,
                        Option::None,
                    ))),
                    Option::Some(_) => {
                        self.menu_visible = true;
                        Option::None
                    }
                };
            }
        }
        return Option::None;
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

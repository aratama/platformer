use crate::body::{play_smash_se, Body};
use crate::geometry::vector2::Vector2;
use crate::graphics::Graphics;
use crate::image::player::PLAYER_IMAGE;
use crate::input::Inputs;
use crate::music::level::LEVEL_BGM_SCORE;
use crate::palette::set_draw_color;
use crate::save::{load, save, GameData, GAME_DATA_VERSION};
use crate::scene::Scene;
use crate::sound::set_bgm;
use crate::wasm4;
use crate::world::{World, CELL_SIZE};
use crate::world_map::WORLD_HEIGHT;
use fastrand::Rng;
use std::str;

use super::ending_scene::EndingScene;

const MIN_PLAYER_LOOKUP: i32 = -60;
const MAX_PLAYER_LOOKUP: i32 = 70;

#[derive(Clone)]
pub struct GameScene {
    rng: Rng,
    frame_count: u32,
    player: Body,
    player_lookup: i32,
    fruits: std::vec::Vec<Body>,
    world: World,
    debug: bool,
    score: f32,
    vibration: i32,
}

impl GameScene {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);

        let world = World::new();

        let player_x = world.start.x;
        let player_y = world.start.y;

        let player = Body::new(
            "player",
            // Vector2::new(CELL_SIZE as f32 * 13.0, CELL_SIZE as f32 * 235.0),
            Vector2::new(player_x, player_y),
            &PLAYER_IMAGE,
            6.0,
            12.0,
        );

        let fruits = vec![
        //     Body::new(
        //     "fruit",
        //     Vector2::new(
        //         rng.i32(0..wasm4::SCREEN_SIZE as i32) as f32,
        //         rng.i32(0..wasm4::SCREEN_SIZE as i32) as f32,
        //     ),
        //     FRUIT_IMAGE,
        //     CELL_SIZE as f32,
        //     CELL_SIZE as f32,
        // )
        ];

        Self {
            frame_count: 0,
            player,
            player_lookup: MIN_PLAYER_LOOKUP,
            fruits,
            rng,
            world,
            debug: false,
            score: 0.0,
            vibration: 0,
        }
    }

    pub fn update(&mut self, inputs: &Inputs) -> Option<Scene> {
        // updates

        self.frame_count += 1;

        self.player.input(&inputs, &self.world);

        self.player
            .physical_update(inputs.horizontal_acceralation() as i32, &self.world);

        if self.player.is_grounded(&self.world)
            && f32::abs(self.player.velocity.x) < 1.0
            && f32::abs(self.player.velocity.y) < 1.0
            && inputs.is_button_pressed(wasm4::BUTTON_UP)
        {
            self.player_lookup = i32::min(MAX_PLAYER_LOOKUP, self.player_lookup + 2);
        } else {
            self.player_lookup = i32::max(MIN_PLAYER_LOOKUP, self.player_lookup - 4);
        }

        for fruit in self.fruits.iter_mut() {
            fruit.physical_update(0, &self.world);
        }

        self.score = f32::max(
            self.score,
            (WORLD_HEIGHT as f32 * CELL_SIZE as f32 - (self.player.position.y)) as f32,
        );

        // if inputs.is_button_just_pressed(wasm4::BUTTON_2) {
        //     // self.debug = !self.debug;
        //     let game_data: GameData = GameData {
        //         version: GAME_DATA_VERSION,
        //         x: self.player.position.x,
        //         y: self.player.position.y,
        //     };
        //     save(&game_data);
        //     let loaded: GameData = load();
        //     wasm4::trace(int_to_string(loaded.x as u32));
        //     wasm4::trace(int_to_string(loaded.y as u32));
        // }

        self.vibration = i32::max(0, self.vibration - 1);

        // renders

        let player_center = self.player.center();
        let dx = wasm4::SCREEN_SIZE as i32 / 2 - player_center.x.floor() as i32;
        let dy = wasm4::SCREEN_SIZE as i32 / 2 - player_center.y.floor() as i32
            + i32::max(0, self.player_lookup);
        let graphics = Graphics {
            frame_count: self.frame_count,
            debug: self.debug,
            dx: dx + (self.vibration as f32 * f32::cos(self.frame_count as f32 * 0.5)) as i32,
            dy,
        };

        // set_draw_color(0x02);
        for i in 0..10 {
            let h = (i * 10) as f32 * CELL_SIZE as f32;
            if self.score < h as f32 {
                // let y = (dy as f32 + ((WORLD_HEIGHT - i * 10) as f32 * CELL_SIZE as f32)) as i32;
                // ここでエラーになる？
                // for x in 0..(wasm4::SCREEN_SIZE / 8) {
                //     wasm4::hline(x as i32 * 8, y, 4);
                // }
                // wasm4::text(int_to_string(i * 10), 1, y + 2);
            }
        }

        set_draw_color(0x3210);
        self.world.draw(graphics);

        self.player.draw(graphics, &self.world, &inputs);

        for fruit in self.fruits.iter() {
            fruit.draw(graphics, &self.world, &inputs);
        }

        // // Stingとの衝突判定
        for sting in self.player.get_stings(&self.world) {
            if sting.intersect(self.player.get_aabb()) {
                play_smash_se();
                const STING_POWER: f32 = 1.0;
                let vec = self.player.position - sting.get_center();
                self.player.velocity.x = if 0.0 < vec.x { 1.0 } else { -1.0 } * 2.5;
                if self.player.is_grounded(&self.world) {
                    self.player.velocity.y = -3.0;
                }
                self.vibration = 16
            }
        }

        // score
        set_draw_color(0x41);
        wasm4::text(int_to_string(self.score as u32 / CELL_SIZE), 0, 0);

        if self.debug {
            set_draw_color(0x41);
            wasm4::text(
                format!(
                    "{0: >04}, {1: >04}",
                    self.player.position.x.floor(),
                    self.player.position.y.floor()
                ),
                0,
                0,
            );
        }

        // bgm
        set_bgm(Option::Some(LEVEL_BGM_SCORE));

        if self.player.position.distance(self.world.carrot) < CELL_SIZE as f32 {
            Option::Some(Scene::EndingScene(EndingScene::new()))
        } else {
            Option::None
        }
    }
}

fn int_to_string(v: u32) -> String {
    fn int_to_char(digit: u32) -> u8 {
        b'0' + (digit as u8)
    }

    let buf: &[u8; 4] = &[
        int_to_char(v / 1000),
        int_to_char(v % 1000 / 100),
        int_to_char(v % 100 / 10),
        int_to_char(v % 10),
    ];
    str::from_utf8(buf).unwrap().to_string()
}

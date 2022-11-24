use crate::body::{play_smash_se, Body};
use crate::geometry::vector2::Vector2;
use crate::graphics::Graphics;
use crate::image::player::PLAYER_IMAGE;
use crate::input::Inputs;
use crate::music::level::LEVEL_BGM_SCORE;
use crate::netplay::{get_my_net_player_index, is_netplay_active};
use crate::palette::set_draw_color;
use crate::save::{load, save, GameData, GAME_DATA_VERSION};
use crate::scene::Scene;
use crate::sound::set_bgm;
use crate::wasm4::trace;
use crate::world::{World, CELL_SIZE};
use crate::world_map::WORLD_HEIGHT;
use crate::{game, wasm4};
use fastrand::Rng;
use std::str;

use super::ending_scene::EndingScene;

const MIN_PLAYER_LOOKUP: i32 = -60;
const MAX_PLAYER_LOOKUP: i32 = 70;

#[derive(Clone)]
pub struct GameScene {
    rng: Rng,
    frame_count: u32,

    players: [Body; 4],

    fruits: std::vec::Vec<Body>,
    world: World,
    debug: bool,
    score: f32,

    prev_gamepads: [u8; 4],
}

impl GameScene {
    pub fn new(player_active: &[bool; 4]) -> Self {
        let rng = Rng::with_seed(235);

        let world = World::new();

        let player_x = world.start.x;
        let player_y = world.start.y;

        let player1 = Body::create_player(0, player_active[0], "player1", player_x, player_y);
        let player2 = Body::create_player(1, player_active[1], "player2", player_x, player_y);
        let player3 = Body::create_player(2, player_active[2], "player3", player_x, player_y);
        let player4 = Body::create_player(3, player_active[3], "player4", player_x, player_y);

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
            players: [player1, player2, player3, player4],
            fruits,
            rng,
            world,
            debug: false,
            score: 0.0,

            prev_gamepads: [0, 0, 0, 0],
        }
    }

    pub fn update(&mut self, inputs: &Inputs) -> Option<Scene> {
        self.frame_count += 1;

        for (i, player) in self.players.iter_mut().enumerate() {
            player.update(&self.world);
        }

        for fruit in self.fruits.iter_mut() {
            fruit.physical_update(0, &self.world);
        }

        // セーブ関係
        if !is_netplay_active() {
            let player1 = &self.players[0];
            self.score = f32::max(
                self.score,
                (WORLD_HEIGHT as f32 * CELL_SIZE as f32 - (player1.position.y)) as f32,
            );

            if inputs.is_button_just_pressed(wasm4::BUTTON_2) {
                // self.debug = !self.debug;
                let game_data: GameData = GameData {
                    version: GAME_DATA_VERSION,
                    x: player1.position.x,
                    y: player1.position.y,
                };
                save(&game_data);
                let loaded: GameData = load();
                wasm4::trace(int_to_string(loaded.x as u32));
                wasm4::trace(int_to_string(loaded.y as u32));
            }
        }

        // renders

        // // Stingとの衝突判定
        // for sting in player1.get_stings(&self.world) {
        //     if sting.intersect(player1.get_aabb()) {
        //         play_smash_se();
        //         const STING_POWER: f32 = 1.0;
        //         let vec = player1.position - sting.get_center();
        //         player1.velocity.x = if 0.0 < vec.x { 1.0 } else { -1.0 } * 2.5;
        //         if player1.is_grounded(&self.world) {
        //             player1.velocity.y = -3.0;
        //         }
        //         self.vibration = 16
        //     }
        // }

        self.render();

        // bgm
        set_bgm(Option::Some(LEVEL_BGM_SCORE));

        // ゴール
        for (i, player) in self.players.iter().enumerate() {
            if player.position.distance(self.world.carrot) < CELL_SIZE as f32 {
                return Option::Some(Scene::EndingScene(EndingScene::new()));
            }
        }
        Option::None
    }

    fn render(&mut self) {
        let my_player = self.players[get_my_net_player_index() as usize];
        let player_center = my_player.center();
        let dx = wasm4::SCREEN_SIZE as i32 / 2 - player_center.x.floor() as i32;
        let dy = wasm4::SCREEN_SIZE as i32 / 2 - player_center.y.floor() as i32
            + i32::max(0, my_player.player_lookup);
        let graphics = Graphics {
            frame_count: self.frame_count,
            debug: self.debug,
            dx: dx + (my_player.vibration as f32 * f32::cos(self.frame_count as f32 * 0.5)) as i32,
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

        for (i, player) in self.players.iter().enumerate() {
            if player.active {
                let gamepad = get_gamepad(i);
                let inptus = Inputs::new(gamepad, self.prev_gamepads[i as usize]);
                player.draw(graphics, &self.world, &inptus);
            }
        }

        // for fruit in self.fruits.iter() {
        //     fruit.draw(graphics, &self.world, &inputs);
        // }

        // score
        set_draw_color(0x41);
        wasm4::text(int_to_string(self.score as u32 / CELL_SIZE), 0, 0);

        if self.debug && !is_netplay_active() {
            let player1 = &self.players[0];
            set_draw_color(0x41);
            wasm4::text(
                format!(
                    "{0: >04}, {1: >04}",
                    player1.position.x.floor(),
                    player1.position.y.floor()
                ),
                0,
                0,
            );
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

pub fn get_gamepad(player_index: usize) -> u8 {
    unsafe {
        match player_index {
            0 => *wasm4::GAMEPAD1,
            1 => *wasm4::GAMEPAD2,
            2 => *wasm4::GAMEPAD3,
            3 => *wasm4::GAMEPAD4,
            _ => 0,
        }
    }
}

use crate::body::Body;
use crate::geometry::vector2::Vector2;
use crate::graphics::Graphics;
use crate::input::Inputs;
use crate::music::level::LEVEL_BGM_SCORE;
use crate::netplay::{get_my_net_player_index, is_netplay_active};
use crate::palette::set_draw_color;
use crate::save::{load, save, GameData, GAME_DATA_VERSION};
use crate::scene::Scene;
use crate::se::play_smash_se;
use crate::sound::set_bgm;
use crate::wasm4::*;
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

    players: [Body; 4],

    fruits: std::vec::Vec<Body>,
    world: World,
    debug: bool,

    prev_gamepads: [u8; 4],
}

impl GameScene {
    pub fn new(game_data: Option<GameData>) -> Self {
        let rng = Rng::with_seed(235);
        let world = World::new();
        let init = Vector2::new(world.start.x, world.start.y);
        let player1 = Body::create_player(
            0,
            "player1",
            game_data.map(|data| data.player1_position).unwrap_or(init),
        );
        let player2 = Body::create_player(
            1,
            "player2",
            game_data.map(|data| data.player2_position).unwrap_or(init),
        );
        let player3 = Body::create_player(
            2,
            "player3",
            game_data.map(|data| data.player3_position).unwrap_or(init),
        );
        let player4 = Body::create_player(
            3,
            "player4",
            game_data.map(|data| data.player4_position).unwrap_or(init),
        );

        let fruits = vec![
        //     Body::new(
        //     "fruit",
        //     Vector2::new(
        //         rng.i32(0..SCREEN_SIZE as i32) as f32,
        //         rng.i32(0..SCREEN_SIZE as i32) as f32,
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

            prev_gamepads: [0, 0, 0, 0],
        }
    }

    pub fn update(&mut self, inputs: &Inputs, player_active: &[bool; 4]) -> Option<Scene> {
        self.frame_count += 1;

        for (index, active) in player_active.iter().enumerate() {
            if *active {
                self.players[index].update(&self.world);
            }
        }

        for fruit in self.fruits.iter_mut() {
            fruit.physical_update(0, &self.world);
        }

        // 毎秒自動セーブする
        if self.frame_count % 60 == 0 {
            // self.debug = !self.debug;
            let game_data: GameData = GameData {
                version: GAME_DATA_VERSION,
                player1_position: self.players[0].position,
                player2_position: self.players[1].position,
                player3_position: self.players[2].position,
                player4_position: self.players[3].position,
            };
            save(&game_data);
        }

        // renders

        // // Stingとの衝突判定
        for player in self.players.iter_mut() {
            for sting in player.get_stings(&self.world) {
                if sting.intersect(player.get_aabb()) {
                    play_smash_se();
                    const STING_POWER: f32 = 1.0;
                    let vec = player.position - sting.get_center();
                    player.velocity.x = if 0.0 < vec.x { 1.0 } else { -1.0 } * 2.5;
                    if player.is_grounded(&self.world) {
                        player.velocity.y = -3.0;
                    }
                    player.vibration = 16
                }
            }
        }

        self.render(player_active);

        // bgm
        set_bgm(Option::Some(LEVEL_BGM_SCORE));

        // ゴール
        for player in self.players.iter() {
            if player.position.distance(self.world.carrot) < CELL_SIZE as f32 {
                return Option::Some(Scene::EndingScene(EndingScene::new()));
            }
        }

        if inputs.is_button_pressed(BUTTON_2) {
            Some(Scene::GameScene(GameScene::new(None)))
        } else {
            Option::None
        }
    }

    fn render(&mut self, player_actives: &[bool; 4]) {
        let my_player = self.players[get_my_net_player_index() as usize];
        let player_center = my_player.center();
        let dx = SCREEN_SIZE as i32 / 2 - player_center.x.floor() as i32;
        let dy = SCREEN_SIZE as i32 / 2 - player_center.y.floor() as i32
            + i32::max(0, my_player.player_lookup);
        let graphics = Graphics {
            frame_count: self.frame_count,
            debug: self.debug,
            dx: dx + (my_player.vibration as f32 * f32::cos(self.frame_count as f32 * 0.5)) as i32,
            dy,
        };

        set_draw_color(0x3210);
        self.world.draw(graphics);

        for (i, active) in player_actives.iter().enumerate() {
            if *active {
                let inptus = Inputs::new(i);
                self.players[i].draw(graphics, &self.world, &inptus);
            }
        }

        // for fruit in self.fruits.iter() {
        //     fruit.draw(graphics, &self.world, &inputs);
        // }

        // score
        set_draw_color(0x4);
        rect(0, 0, 160, 8);
        rect(0, 151, 160, 9);
        if player_actives[0] {
            draw_score("1", self.players[0], 0);
        }
        if player_actives[1] {
            draw_score("2", self.players[1], 40);
        }
        if player_actives[2] {
            draw_score("3", self.players[2], 80);
        }
        if player_actives[3] {
            draw_score("4", self.players[3], 120);
        }

        // stopwatch
        let hours = self.frame_count / 216000;
        let mins = (self.frame_count - 216000 * hours) / 3600;
        let secs = (self.frame_count - 216000 * hours - 3600 * mins) / 60;
        let frames = self.frame_count - 216000 * hours - 3600 * mins - 60 * secs;
        let l = 72;
        draw_digits(hours, l, 0);
        text(":", l + 16, 0);
        draw_digits(mins, l + 24, 0);
        text(":", l + 40, 0);
        draw_digits(secs, l + 48, 0);
        text("'", l + 64, 0);
        draw_digits(frames, l + 72, 0);

        // debug draw
        if self.debug && !is_netplay_active() {
            let player1 = &self.players[0];
            set_draw_color(0x41);
            text(
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

fn draw_score(index: &str, player: Body, x: i32) {
    let score = WORLD_HEIGHT - (player.position.y / CELL_SIZE as f32) as u32;
    set_draw_color(0x24);
    text(index, x, 152);
    set_draw_color(0x41);
    text(int_to_string(score), x + 8, 152);
}

fn draw_digit(d: u32, x: i32, y: i32) {
    text(
        match d {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            _ => "X",
        },
        x,
        y,
    );
}

fn draw_digits(d: u32, x: i32, y: i32) {
    draw_digit(d / 10, x, y);
    draw_digit(d % 10, x + 8, y);
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
    let t = str::from_utf8(buf).unwrap_or("9999").to_string();
    t
}

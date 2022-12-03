use crate::{geometry::vector2::Vector2, wasm4::*};
use bytes::{Buf, BufMut};
use std::mem::size_of;

pub const GAME_DATA_VERSION: u8 = 2;

#[rustfmt::skip]
pub const GAME_DATA_SIZE: usize = 
    size_of::<u8>() + // バージョン
    size_of::<f32>() + // プレイヤー1のx座標
    size_of::<f32>() + // プレイヤー1のy座標
    size_of::<f32>() + // プレイヤー2のx座標
    size_of::<f32>() + // プレイヤー2のy座標
    size_of::<f32>() + // プレイヤー3のx座標
    size_of::<f32>() + // プレイヤー4のy座標
    size_of::<f32>() + // プレイヤー4のx座標
    size_of::<f32>(); // プレイヤー4のy座標

#[derive(Clone, Copy, Debug)]
pub struct GameData {
    pub version: u8,
    pub player1_position: Vector2,
    pub player2_position: Vector2,
    pub player3_position: Vector2,
    pub player4_position: Vector2,
}

pub fn save(game_data: &GameData) {
    let mut buf = Vec::new();
    buf.put_u8(GAME_DATA_VERSION); // version
    buf.put_f32_le(game_data.player1_position.x);
    buf.put_f32_le(game_data.player1_position.y);
    buf.put_f32_le(game_data.player2_position.x);
    buf.put_f32_le(game_data.player2_position.y);
    buf.put_f32_le(game_data.player3_position.x);
    buf.put_f32_le(game_data.player3_position.y);
    buf.put_f32_le(game_data.player4_position.x);
    buf.put_f32_le(game_data.player4_position.y);
    let bytes: &[u8] = buf.as_slice().try_into().unwrap();
    unsafe {
        diskw(bytes.as_ptr(), core::mem::size_of_val(bytes) as u32);
    }
}

/**
 * ゲームデータを読み込みます
 * バージョンを確認し、バージョンが異なる場合は None を返します
 */
pub fn load() -> Option<GameData> {
    let mut buffer: [u8; GAME_DATA_SIZE] = [0; GAME_DATA_SIZE];
    unsafe {
        diskr(buffer.as_mut_ptr(), buffer.len() as u32);
    }
    let mut p = &buffer[..];
    let version = p.get_u8();

    if version != GAME_DATA_VERSION {
        Option::None
    } else {
        let player1_x = p.get_f32_le();
        let player1_y = p.get_f32_le();
        let player2_x = p.get_f32_le();
        let player2_y = p.get_f32_le();
        let player3_x = p.get_f32_le();
        let player3_y = p.get_f32_le();
        let player4_x = p.get_f32_le();
        let player4_y = p.get_f32_le();
        if version == 0 {
            Option::None
        } else {
            Option::Some(GameData {
                version,
                player1_position: Vector2::new(player1_x, player1_y),
                player2_position: Vector2::new(player2_x, player2_y),
                player3_position: Vector2::new(player3_x, player3_y),
                player4_position: Vector2::new(player4_x, player4_y),
            })
        }
    }
}

use crate::wasm4::{diskr, diskw, trace};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

pub const GAME_DATA_VERSION: u8 = 0;
pub const GAME_DATA_SIZE: usize = 16;

#[derive(Deserialize, Serialize)]
pub struct GameData {
    pub version: u8,
    pub x: f32,
    pub y: f32,
}

pub fn save(game_data: &GameData) {
    let vec: Vec<u8> = rmp_serde::to_vec(game_data).unwrap();
    let bytes: &[u8] = vec.as_slice().try_into().unwrap();
    unsafe {
        diskw(bytes.as_ptr(), core::mem::size_of_val(bytes) as u32);
    }
    if GAME_DATA_SIZE < core::mem::size_of_val(bytes) {
        trace(format!(
            "[WARNING] save data size {} exceeds read buffer size {}",
            core::mem::size_of_val(bytes) as u32,
            GAME_DATA_SIZE
        ));
    }
}

pub fn load() -> GameData {
    let mut buffer: [u8; GAME_DATA_SIZE] = [0; GAME_DATA_SIZE];

    unsafe {
        diskr(buffer.as_mut_ptr(), buffer.len() as u32);
    }
    rmp_serde::from_slice(&buffer).unwrap()
}

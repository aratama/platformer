use crate::wasm4::{diskr, diskw};

use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Deserialize, Serialize)]
pub struct GameData {
    pub x: f32,
    pub y: f32,
}

pub fn save(game_data: &GameData) {
    unsafe {
        let vec: Vec<u8> = rmp_serde::to_vec(game_data).unwrap();
        let bytes: &[u8] = vec.as_slice().try_into().unwrap();
        diskw(bytes.as_ptr(), core::mem::size_of_val(bytes) as u32);
    }
}

pub fn load() -> GameData {
    unsafe {
        let mut buffer = [0u8; 1024];
        diskr(buffer.as_mut_ptr(), buffer.len() as u32);
        rmp_serde::from_slice(&buffer).unwrap()
    }
}

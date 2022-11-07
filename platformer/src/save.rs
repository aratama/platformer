use crate::wasm4::{diskr, diskw};
use bytes::{Buf, BufMut};
use std::mem::size_of;

pub const GAME_DATA_VERSION: u8 = 0;

#[rustfmt::skip]
pub const GAME_DATA_SIZE: usize = 
    size_of::<u8>() + // version
    size_of::<f32>() + // x
    size_of::<f32>(); // y

#[derive(Clone, Copy)]
pub struct GameData {
    pub version: u8,
    pub x: f32,
    pub y: f32,
}

pub fn save(game_data: &GameData) {
    let mut buf = Vec::new();
    buf.put_u8(0); // version
    buf.put_f32_le(game_data.x); // x
    buf.put_f32_le(game_data.y); // y
    let bytes: &[u8] = buf.as_slice().try_into().unwrap();
    unsafe {
        diskw(bytes.as_ptr(), core::mem::size_of_val(bytes) as u32);
    }
}

pub fn load() -> GameData {
    let mut buffer: [u8; GAME_DATA_SIZE] = [0; GAME_DATA_SIZE];
    unsafe {
        diskr(buffer.as_mut_ptr(), buffer.len() as u32);
    }
    let mut p = &buffer[..];
    let version = p.get_u8();
    let x = p.get_f32_le();
    let y = p.get_f32_le();
    GameData { version, x, y }
}

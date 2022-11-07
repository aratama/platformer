pub const GAME_DATA_VERSION: u8 = 0;
pub const GAME_DATA_SIZE: usize = 16;

#[derive(Clone, Copy)]
pub struct GameData {
    pub version: u8,
    pub x: f32,
    pub y: f32,
}

pub fn save(_game_data: &GameData) {}

pub fn load() -> GameData {
    GameData {
        version: 0,
        x: 0.0,
        y: 0.0,
    }
}

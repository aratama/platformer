use self::{ending_scene::EndingScene, game_scene::GameScene, title_scene::TitleScene};

pub mod ending_scene;
pub mod game_scene;
pub mod title_scene;

#[derive(Clone)]
pub enum Scene {
    TitleScene(TitleScene),
    GameScene(GameScene),
    EndingScene(EndingScene),
}

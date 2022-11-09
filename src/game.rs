use crate::{game_scene::GameScene, title_scene::TitleScene};

pub struct Game {
    scene: Scene,
    title_scene: TitleScene,
    game_scene: GameScene,
}

pub enum Scene {
    TitleScene,
    GameScene,
}

impl Game {
    pub fn new() -> Self {
        Game {
            scene: Scene::TitleScene,
            title_scene: TitleScene::new(),
            game_scene: GameScene::new(),
        }
    }

    pub fn update(&mut self) {
        self.scene = match &self.scene {
            Scene::TitleScene => self.title_scene.update(),
            Scene::GameScene => self.game_scene.update(),
        };
    }
}

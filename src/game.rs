use crate::scene::ending_scene::EndingScene;
use crate::scene::game_scene::GameScene;
use crate::scene::title_scene::TitleScene;
use crate::scene::Scene;

pub struct Game {
    scene: Scene,
    title_scene: TitleScene,
    game_scene: GameScene,
    ending_scene: EndingScene,
}

impl Game {
    pub fn new() -> Self {
        Game {
            scene: Scene::TitleScene,
            title_scene: TitleScene::new(),
            game_scene: GameScene::new(),
            ending_scene: EndingScene::new(),
        }
    }

    pub fn update(&mut self) {
        self.scene = match &self.scene {
            Scene::TitleScene => self.title_scene.update(),
            Scene::GameScene => self.game_scene.update(),
            Scene::EndingScene => self.ending_scene.update(),
        };
    }
}

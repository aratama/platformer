use crate::game::Scene;
use crate::image::title::TITLE_IMAGE;
use crate::input::Inputs;
use crate::palette::{set_draw_color, set_palette};
use crate::wasm4::*;
pub struct TitleScene {
    prev_gamepad: u8,
}

impl TitleScene {
    pub fn new() -> Self {
        TitleScene { prev_gamepad: 0 }
    }

    pub fn update(&mut self) -> Scene {
        let gamepad = unsafe { *GAMEPAD1 };
        let inputs = Inputs::new(gamepad, self.prev_gamepad);

        // w4 watchで読み込めない？
        // ビルド済みのものを w4 run で読み込む場合は動作する
        set_draw_color(0x4321);
        // blit(
        //     TITLE_IMAGE.data,
        //     0,
        //     0,
        //     TITLE_IMAGE.width,
        //     TITLE_IMAGE.height,
        //     TITLE_IMAGE.flags,
        // );

        text("TOWER CLIMBER", 30, 60);
        text("PRESS ANY KEY", 30, 110);
        text("TO START", 50, 120);

        self.prev_gamepad = unsafe { *GAMEPAD1 };

        if inputs.is_any_button_just_pressed() {
            Scene::GameScene
        } else {
            Scene::TitleScene
        }
    }
}

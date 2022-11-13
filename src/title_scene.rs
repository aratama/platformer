use crate::game::Scene;
use crate::input::Inputs;
use crate::wasm4::*;

#[cfg(not(debug_assertions))]
use crate::image::title::TITLE_IMAGE;
#[cfg(not(debug_assertions))]
use crate::palette::set_draw_color;

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

        // なぜか w4 watch でリンクエラーになるため、デバッグ時とリリース時で画像を出し分ける
        // cargo build でビルド済みのものを w4 run で読み込む場合は動作する
        // よくわからないが https://github.com/rust-lang/rust/issues/46645#issuecomment-423912553
        // で言われている問題？
        self.draw_title_image();

        self.prev_gamepad = unsafe { *GAMEPAD1 };

        if inputs.is_any_button_just_pressed() {
            Scene::GameScene
        } else {
            Scene::TitleScene
        }
    }

    #[cfg(debug_assertions)]
    fn draw_title_image(&self) {
        text("TOWER CLIMBER", 30, 60);
        text("PRESS ANY KEY", 30, 110);
        text("TO START", 50, 120);
    }

    #[cfg(not(debug_assertions))]
    fn draw_title_image(&self) {
        set_draw_color(0x4321);
        blit(
            TITLE_IMAGE.data,
            0,
            0,
            TITLE_IMAGE.width,
            TITLE_IMAGE.height,
            TITLE_IMAGE.flags,
        );
    }
}

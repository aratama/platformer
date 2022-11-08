use crate::image::walk0::WALK0_IMAGE;
use crate::image::walk1::WALK1_IMAGE;
use crate::image::walk2::WALK2_IMAGE;
use crate::image::walk3::WALK3_IMAGE;

use crate::image::Image;

pub const WALK_ANIMATION_WIDTH: u32 = 8;
pub const WALK_ANIMATION_HEIGHT: u32 = 16;
pub const WALK_ANIMATION: Image = Image {
    width: 8,
    height: 16,
    frames: &[
        &WALK0_IMAGE.frames[0],
        &WALK1_IMAGE.frames[0],
        &WALK2_IMAGE.frames[0],
        &WALK3_IMAGE.frames[0],
    ],
    flags: WALK0_IMAGE.flags,
};

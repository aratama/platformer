use crate::animation::Animation;
use crate::image::walk0::WALK0_IMAGE;
use crate::image::walk1::WALK1_IMAGE;
use crate::image::walk2::WALK2_IMAGE;
use crate::image::walk3::WALK3_IMAGE;

pub const WALK_ANIMATION_WIDTH: u32 = 8;
pub const WALK_ANIMATION_HEIGHT: u32 = 16;
pub const WALK_ANIMATION: Animation = Animation {
    frames: &[&WALK0_IMAGE, &WALK1_IMAGE, &WALK2_IMAGE, &WALK3_IMAGE],
};

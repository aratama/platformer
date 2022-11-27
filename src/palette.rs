use crate::wasm4::*;

pub fn set_draw_color(idx: u16) {
    unsafe { *DRAW_COLORS = idx }
}

pub fn set_palette(palette: [u32; 4]) {
    unsafe {
        *PALETTE = palette;
    }
}

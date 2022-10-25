use crate::vector2::Vector2;
use crate::wasm4;

#[derive(Clone, Copy)]
pub struct AABB {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl AABB {
    pub fn r(&self) -> f32 {
        self.x + self.w
    }

    pub fn b(&self) -> f32 {
        self.y + self.h
    }

    // https://developer.mozilla.org/ja/docs/Games/Techniques/3D_collision_detection
    pub fn is_point_inside(&self, point: Vector2) -> bool {
        (point.x >= self.x && point.x <= self.r()) && (point.y >= self.y && point.y <= self.b())
    }

    /**
     * 隣接する場合を含みます
     */
    pub fn intersect(&self, b: AABB) -> bool {
        (self.x <= b.r() && self.r() >= b.x) && (self.y <= b.b() && self.b() >= b.y)
    }

    /**
     * 重なる部分が
     */
    pub fn collesion(&self, b: AABB) -> bool {
        (self.x < b.r() && self.r() > b.x) && (self.y < b.b() && self.b() > b.y)
    }

    pub fn translate(&self, dx: f32, dy: f32) -> AABB {
        AABB {
            x: self.x + dx,
            y: self.y + dy,
            w: self.w,
            h: self.h,
        }
    }
}

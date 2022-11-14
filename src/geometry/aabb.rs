use crate::geometry::vector2::Vector2;

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

    pub fn get_center(&self) -> Vector2 {
        Vector2::new(self.x + self.w * 0.5, self.y + self.h * 0.5)
    }

    // https://developer.mozilla.org/ja/docs/Games/Techniques/3D_collision_detection
    pub fn is_point_inside(&self, point: Vector2) -> bool {
        (point.x >= self.x && point.x <= self.r()) && (point.y >= self.y && point.y <= self.b())
    }

    /**
     * 交差判定します。隣接する場合を含みます
     */
    pub fn intersect(&self, b: AABB) -> bool {
        (self.x <= b.r() && self.r() >= b.x) && (self.y <= b.b() && self.b() >= b.y)
    }

    /**
     * 重なる部分あるか判定します
     */
    pub fn collesion(&self, b: AABB) -> bool {
        (self.x < b.r() && self.r() > b.x) && (self.y < b.b() && self.b() > b.y)
    }

    pub fn collections(&self, walls: &Vec<AABB>) -> bool {
        for wall in walls.iter() {
            if self.collesion(*wall) {
                return true;
            }
        }
        false
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

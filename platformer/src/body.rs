use crate::aabb::AABB;
use crate::graphics::Graphics;
use crate::image::jump::JUMP_IMAGE;
use crate::image::lie::LIE_IMAGE;
use crate::image::lookup::LOOKUP_IMAGE;
use crate::image::walk::WALK_ANIMATION;
use crate::image::Image;
use crate::input::Inputs;
use crate::vector2::Vector2;
use crate::wasm4;
use crate::world::{World, CELL_SIZE};

const MAX_VELOCITY: f32 = 100.0;
const JUMP_VELOCITY: f32 = 2.5;
const WALK_ACCELERATION: f32 = 0.2;
const JUMP_MARGIN: f32 = 0.3;
const MAX_HORIZONTAL_SPEED: f32 = 2.0;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right,
}

pub enum Pose {
    Stand,
    Lie,
    LookUp,
}

pub struct Body {
    pub name: &'static str,
    pub position: Vector2,
    pub velocity: Vector2,
    pub image: Image,
    pub direction: Direction,
    pub pose: Pose,
}

impl Body {
    pub fn new(name: &'static str, position: Vector2, image: Image) -> Self {
        Self {
            name,
            position,
            velocity: Vector2::default(),
            image,
            direction: Direction::Right,
            pose: Pose::Stand,
        }
    }

    pub fn draw(&self, g: Graphics, world: &World, inputs: &Inputs) {
        if g.debug {
            if (g.frame_count / 8) % 2 == 0 {
                g.set_draw_color(0x20);
            } else {
                g.set_draw_color(0x10);
            }
            g.rect(
                self.position.x as i32,
                self.position.y as i32,
                CELL_SIZE,
                CELL_SIZE,
            );
        }

        let flags = wasm4::BLIT_2BPP
            | if self.direction == Direction::Right {
                0
            } else {
                wasm4::BLIT_FLIP_X
            };

        let grounded = self.is_grounded(world);
        let i: Image = if grounded {
            match self.pose {
                Pose::Stand => self.image,
                Pose::Lie => LIE_IMAGE,
                Pose::LookUp => LOOKUP_IMAGE,
            }
        } else {
            JUMP_IMAGE
        };
        let x = (4.0 - i.width as f32 * 0.5 + self.position.x.floor()) as i32;
        let y = (8.0 - i.height as f32 + self.position.y.floor()) as i32;

        if grounded && inputs.is_button_pressed(wasm4::BUTTON_DOWN) {
            g.draw(LIE_IMAGE, x, y, flags);
        } else if grounded && 0.1 < f32::abs(self.velocity.x) {
            g.animate(WALK_ANIMATION, x, y, flags, 5);
        } else {
            g.draw(i, x, y, flags);
        }
    }

    pub fn update(&mut self, inputs: Inputs, world: &World) {
        let gravity: Vector2 = Vector2::new(0.0, 0.1);

        self.velocity.y = f32::max(
            -MAX_VELOCITY,
            f32::min(MAX_VELOCITY, self.velocity.y + gravity.y),
        );
        self.velocity.x = f32::max(
            -MAX_VELOCITY,
            f32::min(MAX_VELOCITY, self.velocity.x + gravity.x),
        );

        self.delta(self.velocity.x, self.velocity.y, world);

        self.pose = if inputs.is_button_pressed(wasm4::BUTTON_DOWN) {
            Pose::Lie
        } else if inputs.is_button_pressed(wasm4::BUTTON_UP) {
            Pose::LookUp
        } else {
            Pose::Stand
        }
    }

    pub fn is_grounded(&self, world: &World) -> bool {
        let walls = self.get_walls(world);
        let aabb = AABB {
            x: self.position.x,
            y: self.position.y,
            w: CELL_SIZE as f32,
            h: CELL_SIZE as f32 + JUMP_MARGIN,
        };
        aabb.collections(&walls)
    }

    fn get_walls(&self, world: &World) -> Vec<AABB> {
        let px = (self.position.x / CELL_SIZE as f32).floor() as i32;
        let py = (self.position.y / CELL_SIZE as f32).floor() as i32;
        let mut walls: Vec<AABB> = vec![];
        // MARGIN = 1 の範囲だと、境界の部分に来たときに衝突判定が遅れて、壁と重なってしまう
        const MARGIN: i32 = 2;
        for cx in (px - MARGIN)..(px + 1 + MARGIN) {
            for cy in (py - MARGIN)..(py + 1 + MARGIN) {
                let cell = world.get_cell(cx, cy);
                if cell != 0 {
                    walls.push(AABB {
                        x: CELL_SIZE as f32 * cx as f32,
                        y: CELL_SIZE as f32 * cy as f32,
                        w: CELL_SIZE as f32,
                        h: CELL_SIZE as f32,
                    })
                }
            }
        }
        walls
    }

    fn delta(&mut self, vx: f32, vy: f32, world: &World) {
        // 壁となるAABBを集める
        let walls = self.get_walls(world);

        let mut aabb = AABB {
            x: self.position.x,
            y: self.position.y,
            w: CELL_SIZE as f32,
            h: CELL_SIZE as f32,
        };

        // 垂直方向に衝突判定
        if vy != 0.0 {
            aabb = aabb.translate(0.0, vy);
            for wall in walls.iter() {
                if aabb.collesion(*wall) {
                    aabb.y = if 0.0 < vy {
                        f32::min(aabb.y + aabb.h, wall.y) - aabb.h
                    } else {
                        f32::max(aabb.y, wall.b())
                    };
                    self.velocity.y = 0.0;
                }
            }
        }

        // 水平方向に衝突判定
        if vx != 0.0 {
            aabb = aabb.translate(vx, 0.0);
            for wall in walls.iter() {
                if aabb.collesion(*wall) {
                    aabb.x = if 0.0 < vx {
                        f32::min(aabb.x + aabb.w, wall.x) - aabb.w
                    } else {
                        f32::max(aabb.x, wall.r())
                    };
                    self.velocity.x = 0.0;
                }
            }
        }

        self.position.x = aabb.x;
        self.position.y = aabb.y;
    }

    pub fn jump(&mut self, world: &World) {
        if self.is_grounded(world) {
            self.pose = Pose::Stand;
            self.velocity.y = -JUMP_VELOCITY
        }
    }

    pub fn walk(&mut self, speed: f32) {
        // 吹き飛ばされた場合など、すでにMAX_HORIZONTAL_SPEEDを超える速度が出ている場合は、
        // それ以上加速はしないものの、MAX_HORIZONTAL_SPEED以下に抑えることもしない
        if self.velocity.x < MAX_HORIZONTAL_SPEED && -MAX_HORIZONTAL_SPEED < self.velocity.x {
            // 歩きで加速する場合はMAX_HORIZONTAL_SPEEDを超えない
            self.velocity.x += f32::max(
                -MAX_HORIZONTAL_SPEED,
                f32::min(MAX_HORIZONTAL_SPEED, WALK_ACCELERATION * speed),
            );
        }
    }

    pub fn input(&mut self, input: Inputs, world: &World) {
        let grounded = self.is_grounded(&world);
        let speed_scale = if grounded { 1.0 } else { 0.4 };

        if !input.is_button_pressed(wasm4::BUTTON_DOWN) {
            if input.is_button_pressed(wasm4::BUTTON_LEFT) {
                if grounded {
                    self.direction = Direction::Left;
                }
                self.walk(-speed_scale);
            }
            if input.is_button_pressed(wasm4::BUTTON_RIGHT) {
                if grounded {
                    self.direction = Direction::Right;
                }
                self.walk(speed_scale);
            }
        }

        if !input.is_button_pressed(wasm4::BUTTON_LEFT)
            && !input.is_button_pressed(wasm4::BUTTON_RIGHT)
        {
            self.velocity.x = self.velocity.x * 0.8;
        }

        if input.is_button_just_pressed(wasm4::BUTTON_1) {
            self.jump(world)
        }
    }
}

use crate::aabb::AABB;
use crate::graphics::Graphics;
use crate::image::jump::JUMP_IMAGE;
use crate::image::lie::LIE_IMAGE;
use crate::image::lookup::LOOKUP_IMAGE;
use crate::image::Image;
use crate::input::Inputs;
use crate::vector2::Vector2;
use crate::wasm4;
use crate::world::World;

const MAX_VELOCITY: f32 = 100.0;
const JUMP_VELOCITY: f32 = 2.5;
const WALK_VELOCITY: f32 = 1.0;
const CRAWL_VELOCITY: f32 = 0.25;
const JUMP_MARGIN: f32 = 0.3;

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

    pub fn draw(&self, g: Graphics, world: &World) {
        if g.debug {
            if (g.frame_count / 8) % 2 == 0 {
                g.set_draw_color(0x20);
            } else {
                g.set_draw_color(0x10);
            }
            g.rect(self.position.x as i32, self.position.y as i32, 8, 8);
        }

        let flags = wasm4::BLIT_2BPP
            | if self.direction == Direction::Right {
                0
            } else {
                wasm4::BLIT_FLIP_X
            };

        let i: Image = if self.is_grounded(world) {
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
        g.draw(i, x, y, flags);
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
            w: 8.0,
            h: 8.0 + JUMP_MARGIN,
        };
        aabb.collections(&walls)
    }

    fn get_walls(&self, world: &World) -> Vec<AABB> {
        let px = (self.position.x / 8.0).floor() as i32;
        let py = (self.position.y / 8.0).floor() as i32;
        let mut walls: Vec<AABB> = vec![];
        // MARGIN = 1 の範囲だと、境界の部分に来たときに衝突判定が遅れて、壁と重なってしまう
        const MARGIN: i32 = 2;
        for cx in (px - MARGIN)..(px + 1 + MARGIN) {
            for cy in (py - MARGIN)..(py + 1 + MARGIN) {
                let cell = world.get_cell(cx, cy);
                if cell != 0 {
                    walls.push(AABB {
                        x: 8.0 * cx as f32,
                        y: 8.0 * cy as f32,
                        w: 8.0,
                        h: 8.0,
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
            w: 8.0,
            h: 8.0,
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

    pub fn walk(&mut self, speed: f32, input: Inputs) {
        self.velocity.x = WALK_VELOCITY
            * speed
            * if input.is_button_pressed(wasm4::BUTTON_DOWN) {
                CRAWL_VELOCITY
            } else {
                1.0
            };
    }

    pub fn input(&mut self, input: Inputs, world: &World) {
        if input.is_button_pressed(wasm4::BUTTON_LEFT) {
            self.direction = Direction::Left;
            self.walk(-1.0, input);
        }
        if input.is_button_pressed(wasm4::BUTTON_RIGHT) {
            self.direction = Direction::Right;
            self.walk(1.0, input);
        }

        if !input.is_button_pressed(wasm4::BUTTON_LEFT)
            && !input.is_button_pressed(wasm4::BUTTON_RIGHT)
        {
            self.velocity.x = 0.0;
        }

        if input.is_button_just_pressed(wasm4::BUTTON_1) {
            self.jump(world)
        }
    }
}

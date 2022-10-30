use crate::aabb::AABB;
use crate::direction::Direction;
use crate::graphics::Graphics;
use crate::image::climb::{CLIMB_HEIGHT, CLIMB_IMAGE, CLIMB_WIDTH};
use crate::image::jump::JUMP_IMAGE;
use crate::image::lie::LIE_IMAGE;
use crate::image::lookup::LOOKUP_IMAGE;
use crate::image::player::PLAYER_IMAGE;
use crate::image::walk::WALK_ANIMATION;
use crate::image::Image;
use crate::input::Inputs;
use crate::vector2::Vector2;
use crate::wasm4;
use crate::world::{World, CELL_SIZE};

// 重力
const GRAVITY_X: f32 = 0.0;
const GRAVITY_Y: f32 = 0.15;

// ジャンプによる加速度(の絶対量)
const JUMP_ACCELERATION: f32 = 4.0;

// 左右移動による加速度(の絶対量)
const WALK_ACCELERATION: f32 = 0.4;

// 足元から接地とみなす範囲をひろげる量
// 完全に設置していないとジャンプできないようにすると連続ジャンプが難しくなるため、ある程度マージンを設ける
const JUMP_MARGIN: f32 = 0.3;

// 速度が大きくなりすぎると衝突判定などが不安定になるため、最大速度を設ける
const MAX_SPEED_X: f32 = 10.0;

// 吹き飛ばされた場合など、すでにMAX_HORIZONTAL_SPEEDを超える速度が出ている場合は、
// それ以上加速はしないものの、MAX_HORIZONTAL_SPEED以下に抑えることもしない
// 歩きで加速する場合はMAX_HORIZONTAL_SPEEDを超えない
const MAX_WALK_SPEED_X: f32 = 2.0;

// 地上に比べて空中での左右キーが効く量の係数
// 空中でもわずかに左右への加速度が調整できる
const AIRIAL_CONTROL: f32 = 0.05;

// 接地中の抵抗（を計算するときの係数）。1.0だと無抵抗で
const GROUND_RESISTANCE_X: f32 = 0.8;

// 空中手の抵抗（を計算するときの係数）
const AIR_RESISTANCE_X: f32 = 1.0;

// 空中ので空気抵抗
// この値で落下中の終端速度も決まる
const AIR_RESISTANCE_Y: f32 = 0.98;

const CLING_MERGIN: f32 = 2.0;

pub struct Body {
    pub name: &'static str,

    // 衝突判定AABBの左上
    pub position: Vector2,

    pub velocity: Vector2,

    pub image: Image,

    pub direction: Direction,

    // 衝突判定のAABBのサイズ
    pub body_width: f32,
    pub body_height: f32,

    // よじ登り中かどうか
    // 0はよじ登りをしていない
    // -1は左向きへよじ登り中、1は右向きへよじ登り中
    pub climbing: i32,

    // よじ登ろうとしている位置(ブロック位置ではなく)
    // 現在掴まっている位置を除外して判定しないと、よじ登り中にジャンプしようとしたときに、直後に同じ箇所に掴まってしまう
    // 現在掴まっているのと別の位置である場合のみ、新たに掴まり判定が有効になる
    pub climbing_point: Option<Vector2>,

    pub wait: i32,
}

fn sign(v: i32) -> i32 {
    if 0 < v {
        1
    } else if v < 0 {
        -1
    } else {
        0
    }
}

impl Body {
    pub fn new(
        name: &'static str,
        position: Vector2,
        image: Image,
        body_width: f32,
        body_height: f32,
    ) -> Self {
        Self {
            name,
            position,
            velocity: Vector2::default(),
            image,
            direction: Direction::Right,
            body_width,
            body_height,
            climbing: 0,
            climbing_point: None,
            wait: 0,
        }
    }

    /**
     * 物体としての状態更新を行います
     * 自由落下や押し出しなど
     * プレイヤーキャラクターとしての操作は input メソッドで更新します
     * プレイヤーの状態によってはこの関数を呼ばないこともあります
     */
    pub fn physical_update(&mut self, cling: i32, world: &World) {
        let grounded = self.is_grounded(world);
        let gravity: Vector2 = Vector2::new(GRAVITY_X, GRAVITY_Y);

        // 重力加速度を加算
        self.velocity.y += gravity.y;
        self.velocity.x += gravity.x;

        // 空気抵抗
        self.velocity.x = self.velocity.x
            * if grounded {
                GROUND_RESISTANCE_X
            } else {
                AIR_RESISTANCE_X
            };

        // 壁ずりおちの場合は空気抵抗が増えているように処理する
        if 0.0 < self.velocity.y && 0 < cling && self.is_touching_right(CLING_MERGIN, world) {
            self.velocity.y = self.velocity.y * 0.5;
            self.direction = Direction::Right;
        } else if 0.0 < self.velocity.y && cling < 0 && self.is_touching_left(CLING_MERGIN, world) {
            self.velocity.y = self.velocity.y * 0.5;
            self.direction = Direction::Left;
        } else {
            self.velocity.y = self.velocity.y * AIR_RESISTANCE_Y
        };

        // 最大速度制限
        self.velocity.x = f32::max(-MAX_SPEED_X, f32::min(MAX_SPEED_X, self.velocity.x));

        // 物体を移動
        if self.climbing == 0 {
            self.move_body(self.velocity.x, self.velocity.y, world);
        } else {
            self.move_body(0.0, 0.0, world);
        }
    }

    pub fn is_grounded(&self, world: &World) -> bool {
        let walls = self.get_walls(world);
        let aabb = AABB {
            x: self.position.x,
            y: self.position.y,
            w: self.body_width,
            h: self.body_height + JUMP_MARGIN,
        };
        aabb.collections(&walls)
    }

    pub fn is_touching_right(&self, margin: f32, world: &World) -> bool {
        let walls = self.get_walls(world);
        let aabb = AABB {
            x: self.position.x,
            y: self.position.y,
            w: self.body_width + margin,
            h: self.body_height,
        };
        aabb.collections(&walls)
    }

    pub fn is_touching_left(&self, margin: f32, world: &World) -> bool {
        let walls = self.get_walls(world);
        let aabb = AABB {
            x: self.position.x - margin,
            y: self.position.y,
            w: self.body_width + margin,
            h: self.body_height,
        };
        aabb.collections(&walls)
    }

    pub fn center(&self) -> Vector2 {
        let x = (self.position.x + self.body_width * 0.5);
        let y = (self.position.y + self.body_height * 0.5);
        Vector2::new(x, y)
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

    /**
     * AABBを移動させます
     * 移動先に壁などがある場合はそこで止まります
     */
    fn move_body(&mut self, vx: f32, vy: f32, world: &World) {
        // 壁となるAABBを集める
        let walls = self.get_walls(world);

        let mut aabb = AABB {
            x: self.position.x,
            y: self.position.y,
            w: self.body_width,
            h: self.body_height,
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

    fn jump(&mut self, world: &World) {
        if self.is_grounded(world) {
            self.velocity.y = -JUMP_ACCELERATION;
        }
    }

    pub fn walk(&mut self, speed: f32) {
        // 吹き飛ばされた場合など、すでにMAX_HORIZONTAL_SPEEDを超える速度が出ている場合は、
        // それ以上加速はしないものの、MAX_HORIZONTAL_SPEED以下に抑えることもしない
        if self.velocity.x < MAX_WALK_SPEED_X && -MAX_WALK_SPEED_X < self.velocity.x {
            // 歩きで加速する場合はMAX_HORIZONTAL_SPEEDを超えない
            self.velocity.x += f32::max(
                -MAX_WALK_SPEED_X,
                f32::min(MAX_WALK_SPEED_X, WALK_ACCELERATION * speed),
            );
        }
    }

    /**
     * 入力にしたがってプレイヤーキャラクターとしての更新を行います
     */
    pub fn input(&mut self, input: Inputs, world: &World) {
        if self.wait == 0 {
            let grounded = self.is_grounded(&world);

            let speed_scale = if grounded { 1.0 } else { AIRIAL_CONTROL };

            if self.climbing != 0 {
                let acc = input.horizontal_acceralation() as i32;

                // 壁の方向に20フレーム押し続けるとよじ登る
                if sign(self.climbing) == acc {
                    self.climbing += 2 * (input.horizontal_acceralation() as i32);
                    if 20 < i32::abs(self.climbing) {
                        self.position.x +=
                            self.body_width as f32 * 0.5 * sign(self.climbing) as f32;
                        self.position.y -= self.body_height as f32 * 0.5;
                        self.climbing = 0;
                        self.climbing_point = None;
                        self.wait = 20;
                    }
                }

                // 移動ボタンを話したらリセットされるようにself.climbingを減らす
                if self.climbing != 0 {
                    if 0 < self.climbing {
                        self.climbing = i32::max(1, self.climbing - sign(self.climbing));
                    } else {
                        self.climbing = i32::min(-1, self.climbing - sign(self.climbing));
                    }
                }

                if input.is_button_just_pressed(wasm4::BUTTON_DOWN) {
                    // 手を放す
                    // self.position.x -= sign(self.climbing) as f32 * CELL_SIZE as f32;
                    self.climbing = 0;
                    self.climbing_point = None;
                    self.velocity.x = 0.0;
                    self.velocity.y = 0.0;
                    self.wait = 15;
                }

                // 掴まり中のジャンプは、キー入力方向に跳ねることができる
                if input.is_button_just_pressed(wasm4::BUTTON_1) {
                    self.velocity.y = -JUMP_ACCELERATION;
                    self.velocity.x = 1.0 * input.horizontal_acceralation();
                    self.climbing = 0;
                }
            } else if 0.0 < self.velocity.y
                && input.is_button_pressed(wasm4::BUTTON_RIGHT)
                && self.is_touching_right(CLING_MERGIN, world)
            {
                // 右ずり落ち
                if input.is_button_just_pressed(wasm4::BUTTON_1) {
                    self.velocity.y = -JUMP_ACCELERATION;
                    self.velocity.x = -1.0;
                    self.direction = Direction::Left;
                }
            } else if 0.0 < self.velocity.y
                && input.is_button_pressed(wasm4::BUTTON_LEFT)
                && self.is_touching_left(CLING_MERGIN, world)
            {
                // 左ずり落ち
                if input.is_button_just_pressed(wasm4::BUTTON_1) {
                    self.velocity.y = -JUMP_ACCELERATION;
                    self.velocity.x = 1.0;
                    self.direction = Direction::Right;
                }
            } else {
                // 左右移動
                if !input.is_button_pressed(wasm4::BUTTON_DOWN) {
                    if input.is_button_pressed(wasm4::BUTTON_LEFT) {
                        if grounded {
                            self.direction = Direction::Left;
                        }
                    }
                    if input.is_button_pressed(wasm4::BUTTON_RIGHT) {
                        if grounded {
                            self.direction = Direction::Right;
                        }
                    }

                    self.walk(speed_scale * input.horizontal_acceralation());
                }

                // ジャンプ
                if input.is_button_just_pressed(wasm4::BUTTON_1) {
                    self.jump(world)
                }

                // 空中で上昇中にジャンプボタンを離した場合は急速に加速度を失うことでジャンプ高さを調節できる
                if !grounded && !input.is_button_pressed(wasm4::BUTTON_1) && self.velocity.y < 0.0 {
                    self.velocity.y *= 0.1;
                }

                // 掴みからのジャンプ
                // if self.climbing_point != None && input.is_button_just_pressed(wasm4::BUTTON_1) {
                //     self.climbing_point = None;
                //     self.velocity.x = 0.0;
                //     self.velocity.y = 0.0;
                // }

                // 現在の掴まり位置から十分離れると、その位置に再度掴まれるようになる
                // match self.climbing_point {
                //     None => {}
                //     Some(p) => {
                //         if CELL_SIZE as f32 * 2.0 < p.distance(self.center()) {
                //             self.climbing_point = None;
                //         }
                //     }
                // }
            }

            // よじ登り判定
            let (current_cell_cx, current_cell_cy) = self.get_current_cell(); // 現在のブロック位置
            let next_cell_cx = current_cell_cx + self.direction.delta(); // よじ登る対象のブロック位置
            let next_cell_cy = current_cell_cy;

            let current_cell = world.get_cell(current_cell_cx, current_cell_cy); // プレイヤーがいるブロック
            let next_cell = world.get_cell(next_cell_cx, current_cell_cy); // よじ登る対象のブロック。これが空の場合はよじ登れない
            let up_next_cell = world.get_cell(next_cell_cx, current_cell_cy - 1); // よじ登る対象ブロックの上のブロック。これが空ならよじ登れない
            let up_cell = world.get_cell(current_cell_cx, current_cell_cy - 1); // よじ登る対象ブロックの手前上のブロック。これが空ならよじ登れる
            let down_cell = world.get_cell(current_cell_cx, current_cell_cy + 1); // 現在位置の下のブロック。これが空ならよじ登れる。この判定をしないと、落ちるおそれのない階段状の地形でも毎回掴みが発生してしまう
            let cells_ok = current_cell == 0
                && next_cell != 0
                && up_next_cell == 0
                && up_cell == 0
                && down_cell == 0; //判定対象の4つのブロックの状態が有効かどうか

            if !grounded
                && 0.0 < self.velocity.y // 落下中のみ。この判定をしないと、小さな山を、壁をこすりながらジャンプしたときに掴みが発動してしまう  
                && input.direction() != None
                && cells_ok
                && (match self.climbing_point {
                    None => true,
                    Some(p) => true,
                })
            {
                self.climbing = input.horizontal_acceralation() as i32;
                self.climbing_point =
                    Some(Vector2::new(current_cell_cx as f32, current_cell_cy as f32));
                self.position.x = (next_cell_cx * CELL_SIZE as i32) as f32
                    + if self.direction == Direction::Right {
                        -self.body_width
                    } else {
                        (CELL_SIZE as i32) as f32
                    };
                self.position.y = (next_cell_cy * CELL_SIZE as i32) as f32 - self.body_height * 0.5;

                self.velocity.x = 0.0;
                self.velocity.y = 0.0;
            }
        }

        // wasm4::trace(format!("x {}", self.position.x));

        self.wait = i32::max(0, self.wait - 1);
    }

    fn get_current_cell(&self) -> (i32, i32) {
        let current_cell_x =
            ((self.position.x + self.body_width * 0.5) / CELL_SIZE as f32).floor() as i32;
        let current_cell_y =
            ((self.position.y + self.body_height * 0.5) / CELL_SIZE as f32).floor() as i32;
        (current_cell_x, current_cell_y)
    }

    pub fn draw(&self, g: Graphics, world: &World, inputs: &Inputs) {
        let flags = wasm4::BLIT_2BPP
            | if self.direction == Direction::Right {
                0
            } else {
                wasm4::BLIT_FLIP_X
            };

        let grounded = self.is_grounded(world);

        let i: Image = if !grounded {
            JUMP_IMAGE
        } else if inputs.is_button_pressed(wasm4::BUTTON_DOWN) {
            LIE_IMAGE
        } else if inputs.is_button_pressed(wasm4::BUTTON_UP) {
            LOOKUP_IMAGE
        } else {
            PLAYER_IMAGE
        };

        let x = (self.body_width * 0.5 - i.width as f32 * 0.5 + self.position.x.floor()) as i32;
        let y = (self.body_height - i.height as f32 + self.position.y.floor()) as i32;

        if self.climbing != 0 {
            let x = (self.position.x).floor() as i32;
            let y = (self.body_height - CLIMB_HEIGHT as f32 + self.position.y.floor()) as i32;
            if self.direction == Direction::Right {
                // TODO: refactor
                g.draw(CLIMB_IMAGE, x - 2, y + 2, flags);
            } else {
                g.draw(CLIMB_IMAGE, x - 8, y + 2, flags);
            }
        } else if grounded && inputs.is_button_pressed(wasm4::BUTTON_DOWN) {
            g.draw(LIE_IMAGE, x, y, flags);
        } else if grounded && 0.1 < f32::abs(self.velocity.x) {
            g.animate(WALK_ANIMATION, x, y, flags, 5);
        } else {
            g.draw(i, x, y, flags);
        }

        if g.debug {
            if (g.frame_count / 8) % 2 == 0 {
                g.set_draw_color(0x20);
                g.rect(
                    self.position.x as i32,
                    self.position.y as i32,
                    self.body_width.floor() as u32,
                    self.body_height.floor() as u32,
                );
            } else {
                // g.set_draw_color(0x10);
            }
        }
    }
}

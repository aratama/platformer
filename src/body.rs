use crate::animation::walk::WALK_ANIMATION;
use crate::geometry::aabb::AABB;
use crate::geometry::direction::Direction;
use crate::geometry::vector2::Vector2;
use crate::graphics::Graphics;
use crate::image::climb::CLIMB_IMAGE;
use crate::image::jump::JUMP_IMAGE;
use crate::image::lie::LIE_IMAGE;
use crate::image::lookup::LOOKUP_IMAGE;
use crate::image::player::PLAYER_IMAGE;
use crate::image::player_ladder0::PLAYER_LADDER0_IMAGE;
use crate::image::player_ladder1::PLAYER_LADDER1_IMAGE;
use crate::image::slip::SLIP_IMAGE;
use crate::image::Image;
use crate::input::Inputs;
use crate::se::play_jump_se;
use crate::wasm4::*;
use crate::world::{Block, World, CELL_SIZE};
use crate::world_map::{WORLD_HEIGHT, WORLD_WIDTH};

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

const MIN_PLAYER_LOOKUP: i32 = -60;
const MAX_PLAYER_LOOKUP: i32 = 70;

const JUMP_BUTTON: u8 = BUTTON_1;

#[derive(Clone, Copy)]
pub struct Body {
    // [0,3] ゲームパッドの名前 1-4 とずれているので注意
    pub player_index: usize,

    // このプレイヤーが参加しているかどうか
    pub active: bool,

    // オブジェクトの名前。使っていない
    pub name: &'static str,

    // 衝突判定AABBの左上
    pub position: Vector2,

    // 衝突判定のAABBのサイズ
    pub body_width: f32,
    pub body_height: f32,

    pub velocity: Vector2,

    pub direction: Direction,

    // 見上げる量。0を超えるとその分だけ上にスクロールする。プレイヤーキャラクターごとに必要
    pub player_lookup: i32,

    // 視界の振動。プレイヤーキャラクターごとに必要
    pub vibration: i32,

    pub stance: Stance,
}

#[derive(Clone, Copy)]
pub struct CliffHangging {
    hangging: i32,

    // よじ登ろうとしている位置(ブロック位置ではなく)
    // 現在掴まっている位置を除外して判定しないと、よじ登り中にジャンプしようとしたときに、直後に同じ箇所に掴まってしまう
    // 現在掴まっているのと別の位置である場合のみ、新たに掴まり判定が有効になる
    point: Vector2,
}

#[derive(Clone, Copy)]
pub enum Stance {
    // 通常
    Neutral,

    // 0より大きい場合は操作できない。毎フレーム1づつ小さくなる
    Wait(u32),

    // カウントはアニメーション用
    OnLadder(u32),

    // よじ登り中かどうか
    // 0はよじ登りをしていない
    // -1は左向きへよじ登り中、1は右向きへよじ登り中
    CliffHangging(CliffHangging),
}

impl Body {
    pub fn new(
        player_index: usize,
        active: bool,
        name: &'static str,
        position: Vector2,
        body_width: f32,
        body_height: f32,
    ) -> Self {
        Self {
            player_index,
            name,
            position,
            velocity: Vector2::default(),
            direction: Direction::Right,
            body_width,
            body_height,
            player_lookup: MIN_PLAYER_LOOKUP,
            vibration: 0,
            active,
            stance: Stance::Neutral,
        }
    }

    pub fn create_player(
        player_index: usize,
        active: bool,
        name: &'static str,
        player_x: f32,
        player_y: f32,
    ) -> Self {
        Body::new(
            player_index,
            active,
            name,
            Vector2::new(player_x, player_y),
            6.0,
            12.0,
        )
    }

    /**
     * 物体としての状態更新を行います
     * 自由落下や押し出しなど
     * プレイヤーキャラクターとしての操作は input メソッドで更新します
     * プレイヤーの状態によってはこの関数を呼ばないこともあります
     */
    pub fn physical_update(&mut self, cling: i32, world: &World) {
        match self.stance {
            Stance::OnLadder(_) => {}
            Stance::CliffHangging(_) => {}
            _ => {
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
                if 0.0 < self.velocity.y && 0 < cling && self.is_touching_right(CLING_MERGIN, world)
                {
                    self.velocity.y = self.velocity.y * 0.5;
                    self.direction = Direction::Right;
                } else if 0.0 < self.velocity.y
                    && cling < 0
                    && self.is_touching_left(CLING_MERGIN, world)
                {
                    self.velocity.y = self.velocity.y * 0.5;
                    self.direction = Direction::Left;
                } else {
                    self.velocity.y = self.velocity.y * AIR_RESISTANCE_Y
                };

                // 最大速度制限
                self.velocity.x = f32::max(-MAX_SPEED_X, f32::min(MAX_SPEED_X, self.velocity.x));

                // 物体を移動
                match self.stance {
                    Stance::CliffHangging(stance) if stance.hangging == 0 => {
                        self.move_body(0.0, 0.0, world);
                    }
                    _ => {
                        self.move_body(self.velocity.x, self.velocity.y, world);
                    }
                }
            }
        }
    }

    /**
     * ゲームパッド入力などを考慮してこのプレイヤーキャラクターを更新します
     * このメソッドでは重力加速度などの反映は行いません
     */
    pub fn update(&mut self, world: &World) {
        let inputs = Inputs::new(self.player_index);

        if inputs.is_any_button_just_pressed() {
            self.active = true;
        }

        self.input(&inputs, world);

        self.physical_update(inputs.horizontal_acceralation() as i32, world);

        match self.stance {
            Stance::OnLadder(_) => {}
            _ => {
                if self.is_grounded(world)
                    && f32::abs(self.velocity.x) < 1.0
                    && f32::abs(self.velocity.y) < 1.0
                    && inputs.is_button_pressed(BUTTON_UP)
                {
                    self.player_lookup = i32::min(MAX_PLAYER_LOOKUP, self.player_lookup + 2);
                } else {
                    self.player_lookup = i32::max(MIN_PLAYER_LOOKUP, self.player_lookup - 4);
                }
            }
        }

        self.vibration = i32::max(0, self.vibration - 1);

        // 制約
        self.position.x = f32::max(0.0, self.position.x);
        self.position.y = f32::max(0.0, self.position.y);
        self.position.x = f32::min((CELL_SIZE * (WORLD_WIDTH - 1)) as f32, self.position.x);
        self.position.y = f32::min((CELL_SIZE * (WORLD_HEIGHT - 1)) as f32, self.position.y);
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

    /**
     * 右側の壁に触れているかどうか
     */
    pub fn is_touching_right(&self, margin: f32, world: &World) -> bool {
        let walls = self.get_walls(world);
        let aabb = AABB {
            x: self.position.x,
            y: self.position.y,
            w: self.body_width + margin,
            h: self.body_height * 0.5,
        };
        aabb.collections(&walls)
    }

    /**
     * 左側の壁に触れているかどうか
     */
    pub fn is_touching_left(&self, margin: f32, world: &World) -> bool {
        let walls = self.get_walls(world);
        let aabb = AABB {
            x: self.position.x - margin,
            y: self.position.y,
            w: self.body_width + margin,
            h: self.body_height * 0.5,
        };
        aabb.collections(&walls)
    }

    pub fn center(&self) -> Vector2 {
        let x = self.position.x + self.body_width * 0.5;
        let y = self.position.y + self.body_height * 0.5;
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
                if !world.is_empty(cx, cy) {
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
     * プレイヤーキャラクター周辺の、接触する可能性のある針を取得します
     */
    pub fn get_stings(&self, world: &World) -> Vec<AABB> {
        let px = (self.position.x / CELL_SIZE as f32).floor() as i32;
        let py = (self.position.y / CELL_SIZE as f32).floor() as i32;
        let mut walls: Vec<AABB> = vec![];
        // MARGIN = 1 の範囲だと、境界の部分に来たときに衝突判定が遅れて、壁と重なってしまう
        const MARGIN: i32 = 2;
        for cx in (px - MARGIN)..(px + 1 + MARGIN) {
            for cy in (py - MARGIN)..(py + 1 + MARGIN) {
                if world.get_cell(cx, cy) == Block::Sting {
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

    pub fn get_aabb(&self) -> AABB {
        AABB {
            x: self.position.x,
            y: self.position.y,
            w: self.body_width,
            h: self.body_height,
        }
    }

    /**
     * AABBを移動させます
     * 移動先に壁などがある場合はそこで止まります
     */
    fn move_body(&mut self, vx: f32, vy: f32, world: &World) {
        // 壁となるAABBを集める
        let walls = self.get_walls(world);

        let mut aabb = self.get_aabb();

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
            play_jump_se();
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
     * 右向きのずり落ちをしているかどうか
     */
    pub fn is_right_slide(&self, input: &Inputs, world: &World) -> bool {
        0.0 < self.velocity.y
            && input.is_button_pressed(BUTTON_RIGHT)
            && self.is_touching_right(CLING_MERGIN, world)
    }

    /**
     * 左向きのずり落ちをしているかどうか
     */
    pub fn is_left_slide(&self, input: &Inputs, world: &World) -> bool {
        0.0 < self.velocity.y
            && input.is_button_pressed(BUTTON_LEFT)
            && self.is_touching_left(CLING_MERGIN, world)
    }

    /**
     * 入力にしたがってプレイヤーキャラクターとしての更新を行います
     */
    pub fn input(&mut self, input: &Inputs, world: &World) {
        let grounded = self.is_grounded(&world);

        let speed_scale = if grounded { 1.0 } else { AIRIAL_CONTROL };

        match self.stance {
            Stance::Wait(0) => {
                self.stance = Stance::Neutral;
            }

            Stance::Wait(wait) => {
                // 待機中はどんな操作もできない
                self.stance = Stance::Wait(u32::max(0, wait - 1));
            }
            Stance::OnLadder(animation) => {
                // はしご上でのジャンプ
                if input.is_button_just_pressed(JUMP_BUTTON) {
                    self.stance = Stance::Neutral;
                    self.velocity.y = -JUMP_ACCELERATION;
                    self.velocity.x = input.horizontal_acceralation() * 0.5;
                    self.direction = input.direction().unwrap_or(self.direction);
                    play_jump_se();
                } else if input.is_button_pressed(BUTTON_DOWN) {
                    self.move_body(0.0, 0.5, world);
                    match self.get_touching_ladder(world) {
                        None => {
                            self.move_body(0.0, -0.5, world);
                        }
                        Some(_) => {
                            self.stance = Stance::OnLadder(animation + 1);
                        }
                    }
                } else if input.is_button_pressed(BUTTON_UP) {
                    self.move_body(0.0, -0.5, world);
                    match self.get_touching_ladder(world) {
                        None => {
                            self.move_body(0.0, 0.5, world);
                        }
                        Some(_) => {
                            self.stance = Stance::OnLadder(animation + 1);
                        }
                    }
                }
            }
            Stance::CliffHangging(stance) => {
                let acc = input.horizontal_acceralation() as i32;

                // 壁の方向に20フレーム押し続けるとよじ登る
                if i32::signum(stance.hangging) == acc {
                    let next_climbing = 2 * (input.horizontal_acceralation() as i32);
                    if 20 < i32::abs(stance.hangging) {
                        self.position.x +=
                            self.body_width as f32 * 0.5 * i32::signum(stance.hangging) as f32;
                        self.position.y -= self.body_height as f32 * 0.5;
                        self.stance = Stance::Wait(20);
                    } else {
                        self.stance = Stance::CliffHangging(CliffHangging {
                            hangging: next_climbing,
                            point: stance.point,
                        });
                    }
                }

                // 移動ボタンを話したらリセットされるようにself.climbingを減らす
                if 0 < stance.hangging {
                    self.stance = Stance::CliffHangging(CliffHangging {
                        hangging: i32::max(1, stance.hangging - i32::signum(stance.hangging)),
                        point: stance.point,
                    });
                } else if stance.hangging < 0 {
                    self.stance = Stance::CliffHangging(CliffHangging {
                        hangging: i32::min(-1, stance.hangging - i32::signum(stance.hangging)),
                        point: stance.point,
                    });
                }

                if input.is_button_just_pressed(BUTTON_DOWN) {
                    // 手を放す
                    // self.position.x -= sign(self.climbing) as f32 * CELL_SIZE as f32;
                    self.stance = Stance::Wait(15);
                    self.velocity.x = 0.0;
                    self.velocity.y = 0.0;
                }

                // 掴まり中のジャンプは、キー入力方向に跳ねることができる
                if input.is_button_just_pressed(JUMP_BUTTON) {
                    self.velocity.y = -JUMP_ACCELERATION;
                    self.velocity.x = 1.0 * input.horizontal_acceralation();
                    self.stance = Stance::Neutral;
                    self.direction =
                        Direction::from_delta(input.horizontal_acceralation(), self.direction);
                    play_jump_se()
                }
            }
            Stance::Neutral => {
                // はしごと重なっている場合ははしご掴まり状態へ
                let touching_ladder = self.get_touching_ladder(world);

                // trace(format!("neutral {}", touching_ladder == None));

                if input.is_button_just_pressed(BUTTON_UP) && touching_ladder != None {
                    match touching_ladder {
                        Some((x, _)) => {
                            self.stance = Stance::OnLadder(0);
                            self.velocity.x = 0.0;
                            self.velocity.y = 0.0;
                            self.position.x = CELL_SIZE as f32 * x as f32;
                            // self.position.y = CELL_SIZE as f32 * y as f32;
                        }
                        None => {}
                    }
                }
                // 右ずり落ち中の操作
                else if self.is_right_slide(input, world) {
                    // 右ずり落ち
                    if input.is_button_just_pressed(JUMP_BUTTON) {
                        self.velocity.y = -JUMP_ACCELERATION;
                        self.velocity.x = -1.0;
                        self.direction = Direction::Left;
                        play_jump_se()
                    }
                }
                // 左ずり落ち中の操作
                else if self.is_left_slide(input, world) {
                    // 左ずり落ち
                    if input.is_button_just_pressed(JUMP_BUTTON) {
                        self.velocity.y = -JUMP_ACCELERATION;
                        self.velocity.x = 1.0;
                        self.direction = Direction::Right;
                        play_jump_se()
                    }
                }
                // ジャンプ
                else if input.is_button_just_pressed(JUMP_BUTTON) {
                    self.jump(world);
                }
                // 左右移動
                else if !input.is_button_pressed(BUTTON_DOWN) {
                    if input.is_button_pressed(BUTTON_LEFT) {
                        if grounded {
                            self.direction = Direction::Left;
                        }
                    }
                    if input.is_button_pressed(BUTTON_RIGHT) {
                        if grounded {
                            self.direction = Direction::Right;
                        }
                    }

                    self.walk(speed_scale * input.horizontal_acceralation());
                }

                // 空中で上昇中にジャンプボタンを離した場合は急速に加速度を失うことでジャンプ高さを調節できる
                if !grounded && !input.is_button_pressed(JUMP_BUTTON) && self.velocity.y < 0.0 {
                    self.velocity.y *= 0.1;
                }

                // よじ登り判定
                let (current_cell_cx, current_cell_cy) = self.get_current_cell(); // 現在のブロック位置
                let next_cell_cx = current_cell_cx + self.direction.delta(); // よじ登る対象のブロック位置
                let next_cell_cy = current_cell_cy;

                let current_cell = world.is_empty(current_cell_cx, current_cell_cy); // プレイヤーがいるブロック
                let next_cell = world.is_climbable(next_cell_cx, current_cell_cy); // よじ登る対象のブロック。これが空の場合はよじ登れない
                let up_next_cell = world.is_empty(next_cell_cx, current_cell_cy - 1); // よじ登る対象ブロックの上のブロック。これが空ならよじ登れない
                let up_cell = world.is_empty(current_cell_cx, current_cell_cy - 1); // よじ登る対象ブロックの手前上のブロック。これが空ならよじ登れる
                let down_cell = world.is_empty(current_cell_cx, current_cell_cy + 1); // 現在位置の下のブロック。これが空ならよじ登れる。この判定をしないと、落ちるおそれのない階段状の地形でも毎回掴みが発生してしまう
                let cells_ok = current_cell && next_cell && up_next_cell && up_cell && down_cell; //判定対象の4つのブロックの状態が有効かどうか

                if !grounded
                        && 0.0 < self.velocity.y // 落下中のみ。この判定をしないと、小さな山を、壁をこすりながらジャンプしたときに掴みが発動してしまう  
                        && input.direction() != None
                        && cells_ok
                {
                    self.stance = Stance::CliffHangging(CliffHangging {
                        hangging: input.horizontal_acceralation() as i32,
                        point: Vector2::new(current_cell_cx as f32, current_cell_cy as f32),
                    });

                    self.position.x = (next_cell_cx * CELL_SIZE as i32) as f32
                        + if self.direction == Direction::Right {
                            -self.body_width
                        } else {
                            (CELL_SIZE as i32) as f32
                        };
                    self.position.y =
                        (next_cell_cy * CELL_SIZE as i32) as f32 - self.body_height * 0.5;

                    self.velocity.x = 0.0;
                    self.velocity.y = 0.0;
                }
            }
        }

        // trace(format!("x {}", self.position.x));
    }

    fn get_touching_ladder(&self, world: &World) -> Option<(i32, i32)> {
        let center = self.center();
        let x = center.x as i32 / CELL_SIZE as i32;
        let y = center.y as i32 / CELL_SIZE as i32;
        let cell = world.get_cell(x, y);
        if cell == Block::Ladder {
            Some((x, y))
        } else {
            None
        }
    }

    fn get_current_cell(&self) -> (i32, i32) {
        let current_cell_x =
            ((self.position.x + self.body_width * 0.5) / CELL_SIZE as f32).floor() as i32;
        let current_cell_y =
            ((self.position.y + self.body_height * 0.5) / CELL_SIZE as f32).floor() as i32;
        (current_cell_x, current_cell_y)
    }

    pub fn draw(&self, g: Graphics, world: &World, inputs: &Inputs) {
        match self.stance {
            Stance::CliffHangging(_) => {
                let x = (self.position.x).floor() as i32;
                let y =
                    (self.body_height - CLIMB_IMAGE.height as f32 + self.position.y.floor()) as i32;
                if self.direction == Direction::Right {
                    // TODO: refactor
                    g.set_draw_color(0x4320);
                    g.draw(&CLIMB_IMAGE, x - 2, y + 2, self.direction.to_flags());
                } else {
                    g.set_draw_color(0x4320);
                    g.draw(&CLIMB_IMAGE, x - 8, y + 2, self.direction.to_flags());
                }
            }
            _ => {
                let grounded = self.is_grounded(world);

                let i: &Image = match self.stance {
                    Stance::OnLadder(animation) => {
                        if (animation / 8) % 2 == 0 {
                            &PLAYER_LADDER0_IMAGE
                        } else {
                            &PLAYER_LADDER1_IMAGE
                        }
                    }
                    _ => {
                        if !grounded {
                            &JUMP_IMAGE
                        } else if inputs.is_button_pressed(BUTTON_DOWN) {
                            &LIE_IMAGE
                        } else if inputs.is_button_pressed(BUTTON_UP) {
                            &LOOKUP_IMAGE
                        } else {
                            &PLAYER_IMAGE
                        }
                    }
                };

                let x =
                    (self.body_width * 0.5 - i.width as f32 * 0.5 + self.position.x.floor()) as i32;
                let y = (self.body_height - i.height as f32 + self.position.y.floor()) as i32;

                if self.is_right_slide(inputs, world) {
                    g.set_draw_color(0x4320);
                    g.draw(&SLIP_IMAGE, x, y, self.direction.to_flags());
                } else if self.is_left_slide(inputs, world) {
                    g.set_draw_color(0x4320);
                    g.draw(&SLIP_IMAGE, x, y, self.direction.to_flags());
                } else if grounded && inputs.is_button_pressed(BUTTON_DOWN) {
                    g.set_draw_color(0x4320);
                    g.draw(&LIE_IMAGE, x, y, self.direction.to_flags());
                } else if grounded && 0.1 < f32::abs(self.velocity.x) {
                    g.set_draw_color(0x4320);
                    g.animate(&WALK_ANIMATION, x, y, self.direction.to_flags(), 5);
                } else {
                    g.set_draw_color(0x4320);
                    g.draw(&i, x, y, self.direction.to_flags());
                }
            }
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

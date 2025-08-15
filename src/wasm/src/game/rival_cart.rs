#[allow(clippy::all)]
pub mod rival_cart {
    use crate::engine::{Point, Renderer, Velocity};
    use crate::game::cart::cart::CarDirection;
    use crate::game::wall::wall::Wall;
    use crate::game::{CANVAS_WIDTH, CART_START_Y, Piece, STAGE_GOAL};

    const RIVAL_CART_WIDTH: f32 = 20.0;
    const RIVAL_CART_HEIGHT: f32 = 50.0;
    const RIVAL_VELOCITY: f32 = 3.0;
    const COLLISION_CHECK_DISTANCE: f32 = 50.0;
    const LEFT_EDGE: f32 = 100.0;
    const RIGHT_EDGE: f32 = 700.0;

    pub struct RivalCart {
        position: Point,
        velocity: Velocity,
        direction: CarDirection,
        distance: f32, // Rival cart's own distance counter
        walls: Vec<Wall>,
    }

    impl RivalCart {
        pub fn new(_walls: &[Wall], _position: Point, speed: f32, _distance: f32) -> Self {
            RivalCart {
                walls: _walls.to_vec(),
                position: _position,
                velocity: Velocity { x: 0.0, y: speed },
                direction: CarDirection::Normal,
                distance: _distance,
            }
        }
        pub fn update(&mut self, _velocity: Velocity) {
            // Update Walls
            self.walls.iter_mut().for_each(|wall| {
                wall.run(Velocity {
                    x: 0.0,
                    y: self.velocity.y,
                });
            });
            // Update rival's own distance independently
            self.distance += self.velocity.y;

            // Update Y position relative to player
            self.position.y += self.velocity.y - _velocity.y;

            // Check for upcoming walls and adjust path using rival's own distance
            self.check_collision_and_adjust(self.distance);

            // Update X position with calculated horizontal velocity
            let _x: f32 = self.position.x + self.velocity.x;
            if (_x > LEFT_EDGE + RIVAL_CART_WIDTH / 2.0)
                && (_x < RIGHT_EDGE - RIVAL_CART_WIDTH / 2.0)
            {
                self.position.x += self.velocity.x;
            }
        }
        pub fn get_distance(&self) -> f32 {
            self.distance
        }

        fn check_collision_and_adjust(&mut self, distance: f32) {
            let center_x = self.position.x;
            let center_y = distance;
            let left_x = center_x - RIVAL_CART_WIDTH / 2.0;
            let check_distance = 80.0;

            // 1. ライバルカーの中央と中央からcheck_100px先の点を結んだ線と壁の交差チェック
            let _center_ahead_point = Point::new(center_x, center_y + check_distance);
            let _center_point = Point::new(center_x, center_y);
            if !self.line_segments_intersect(_center_point, _center_ahead_point) {
                // 交わらない場合は、まっすぐ進む
                self.velocity.x = 0.0;
                self.direction = CarDirection::Normal;
                return;
            } else {
                // 2. ライバルカーの左端と左端から指定した先の点を結んだ線と壁の交差チェック
                let _left_ahead_point = Point::new(left_x, center_y + check_distance);
                let _left_point = Point::new(left_x, center_y);
                if !self.line_segments_intersect(_left_point, _left_ahead_point) {
                    // 交わらない場合は、左に1px移動
                    self.velocity.x = -4.0;
                    self.direction = CarDirection::Left;
                    return;
                } else {
                    // 3. 左端も交わる場合は、右に1px移動
                    self.velocity.x = 4.0;
                    self.direction = CarDirection::Right;
                }
            }
        }

        fn line_segments_intersect(&self, _p: Point, _q: Point) -> bool {
            for _w in &self.walls {
                log!("cart px:{} py:{} qx:{} qy:{}", _p.x, _p.y, _q.x, _q.y);
                log!(
                    "wall px:{} py:{} qx:{} qy:{}",
                    _w.p().x,
                    _w.p().y,
                    _w.q().x,
                    _w.q().y,
                );
                if ((_p.x - _q.x) * (_w.p().y - _p.y) + (_p.y - _q.y) * (_p.x - _w.p().x))
                    * ((_p.x - _q.x) * (_w.q().y - _p.y) + (_p.y - _q.y) * (_p.x - _w.q().x))
                    < 0.0
                    && ((_w.p().x - _w.q().x) * (_p.y - _w.p().y)
                        + (_w.p().y - _w.q().y) * (_w.p().x - _p.x))
                        * ((_w.p().x - _w.q().x) * (_q.y - _w.p().y)
                            + (_w.p().y - _w.q().y) * (_w.p().x - _q.x))
                        < 0.0
                {
                    log!("TRUE");
                    return true;
                }
            }
            log!("FALSE");
            return false;
        }

        pub fn get_position(&self) -> Point {
            self.position
        }

        pub fn get_velocity(&self) -> Velocity {
            self.velocity
        }

        pub fn check_collision_with_cart(&self, cart_position: Point) -> bool {
            let dx = self.position.x - cart_position.x;
            let dy = self.position.y - cart_position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Collision radius (roughly half the width of both cars)
            let collision_radius = 25.0;
            distance < collision_radius
        }

        pub fn reset(&mut self, _p: Point, _v: Velocity) {
            self.position = _p;
            self.velocity = _v;
            self.direction = CarDirection::Normal;
            self.distance = 0.0;
        }

        pub fn draw(&self, renderer: &Renderer) {
            // Calculate draw position based on rival's position relative to player
            let _draw_position: Point = Point::new(self.position.x, self.position.y);
            // Only draw if rival is visible on screen
            //if draw_position_y > -100.0 && draw_position.y < 1100.0 {
            match self.direction {
                CarDirection::Left => {
                    renderer.draw_blue_left_facing_racing_car(&_draw_position);
                }
                CarDirection::Right => {
                    renderer.draw_blue_right_facing_racing_car(&_draw_position);
                }
                CarDirection::Normal => {
                    renderer.draw_blue_normal_racing_car(&_draw_position);
                }
            }
            //}
        }
    }
}

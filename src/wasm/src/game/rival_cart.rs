#[allow(clippy::all)]
pub mod rival_cart {
    use crate::engine::{Point, Renderer, Velocity};
    use crate::game::cart::cart::CarDirection;
    use crate::game::wall::wall::Wall;
    use crate::game::{Piece, STAGE_GOAL};

    const RIVAL_CART_WIDTH: f32 = 20.0;
    const COLLISION_CHECK_DISTANCE: f32 = 50.0;
    const LEFT_EDGE: f32 = 100.0;
    const RIGHT_EDGE: f32 = 700.0;
    const EVASION_SPEED: f32 = 4.0;

    #[derive(Copy, Clone)]
    pub struct RivalCart {
        position: Point,
        velocity: Velocity,
        direction: CarDirection,
        distance: f32, // Rival cart's own distance counter
        no: usize,
    }

    impl RivalCart {
        pub fn new(_position: Point, speed: f32, _distance: f32, _no: usize) -> Self {
            RivalCart {
                position: _position,
                velocity: Velocity { x: 0.0, y: speed },
                direction: CarDirection::Normal,
                distance: _distance,
                no: _no,
            }
        }
        pub fn update(&mut self, _walls: &[Wall], _velocity: Velocity) {
            // Update rival's own distance independently
            self.distance += self.velocity.y;

            // Update Y position relative to player
            self.position.y += self.velocity.y - _velocity.y;

            // Update X position with calculated horizontal velocity
            let _x: f32 = self.position.x + self.velocity.x;
            if (_x > LEFT_EDGE + RIVAL_CART_WIDTH / 2.0)
                && (_x < RIGHT_EDGE - RIVAL_CART_WIDTH / 2.0)
            {
                self.position.x += self.velocity.x;
            }

            if self.distance > STAGE_GOAL {
                self.distance = 0.0;
            }

            // Check for upcoming walls and adjust path using rival's own distance
            self.check_collision_and_adjust(_walls);
        }

        fn check_collision_and_adjust(&mut self, _walls: &[Wall]) {
            let _center_x = self.position.x;
            let _y = self.position.y;
            let _left_x = _center_x - RIVAL_CART_WIDTH / 2.0 - 10.0;
            let _right_x = _center_x + RIVAL_CART_WIDTH / 2.0 + 10.0;

            // Check the intersection of the line connecting the center of the rival car
            let _left_point = Point::new(_left_x, _y);
            let _left_ahead_point = Point::new(_left_x, _y + COLLISION_CHECK_DISTANCE);
            let _right_point = Point::new(_right_x, _y);
            let _right_ahead_point = Point::new(_right_x, _y + COLLISION_CHECK_DISTANCE);

            if !self.line_segments_intersect(_walls, _right_point, _right_ahead_point)
                && !self.line_segments_intersect(_walls, _left_point, _left_ahead_point)
            {
                //1. If they don't intersect, go straight
                self.velocity.x = 0.0;
                self.direction = CarDirection::Normal;
                return;
            }
            match self.no {
                1 => {
                    // 2. Checks whether the line connecting the left edge of the rival car
                    if !self.line_segments_intersect(_walls, _left_point, _left_ahead_point) {
                        // If they do not intersect, move left
                        self.velocity.x = -EVASION_SPEED;
                        self.direction = CarDirection::Left;
                        return;
                    }
                    // 3.If the left edge also intersects, move to the right
                    self.velocity.x = EVASION_SPEED;
                    self.direction = CarDirection::Right;
                }
                _ => {
                    if !self.line_segments_intersect(_walls, _right_point, _right_ahead_point) {
                        // If they do not intersect, move right
                        self.velocity.x = EVASION_SPEED;
                        self.direction = CarDirection::Right;
                        return;
                    }
                    // If they do not intersect, move left
                    self.velocity.x = -EVASION_SPEED;
                    self.direction = CarDirection::Left;
                }
            }
        }

        fn line_segments_intersect(&self, _walls: &[Wall], _p: Point, _q: Point) -> bool {
            for _w in _walls {
                if _w.p().y.min(_w.q().y) < _p.y && _w.p().y.min(_w.q().y) > _p.y {
                    return false;
                }
                if ((_p.x - _q.x) * (_w.p().y - _p.y) + (_p.y - _q.y) * (_p.x - _w.p().x))
                    * ((_p.x - _q.x) * (_w.q().y - _p.y) + (_p.y - _q.y) * (_p.x - _w.q().x))
                    < 0.0
                    && ((_w.p().x - _w.q().x) * (_p.y - _w.p().y)
                        + (_w.p().y - _w.q().y) * (_w.p().x - _p.x))
                        * ((_w.p().x - _w.q().x) * (_q.y - _w.p().y)
                            + (_w.p().y - _w.q().y) * (_w.p().x - _q.x))
                        < 0.0
                {
                    return true;
                }
            }
            return false;
        }

        pub fn set_position(&mut self, _position: Point) {
            self.position = _position
        }

        pub fn get_position(&self) -> Point {
            self.position
        }

        pub fn get_velocity(&self) -> Velocity {
            self.velocity
        }

        pub fn get_distance(&self) -> f32 {
            self.distance
        }

        pub fn get_no(&self) -> usize {
            self.no
        }

        pub fn check_collision_with_cart(&self, cart_position: Point) -> bool {
            let dx = self.position.x - cart_position.x;
            let dy = self.position.y - cart_position.y;
            let distance = (dx * dx + dy * dy).sqrt();
            let collision_radius = 25.0;
            distance < collision_radius
        }

        pub fn draw(&self, renderer: &Renderer) {
            // Calculate draw position based on rival's position relative to player
            let _draw_position: Point = Point::new(self.position.x, self.position.y);
            // Only draw if rival is visible on screen
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
        }
    }
}

#[allow(clippy::all)]
pub mod rival_cart {
    use crate::engine::{Point, Renderer, Velocity};
    use crate::game::Piece;
    use crate::game::cart::cart::CarDirection;
    use crate::game::wall::wall::Wall;

    const RIVAL_CART_WIDTH: f32 = 20.0;
    const RIVAL_CART_HEIGHT: f32 = 50.0;
    const RIVAL_VELOCITY: f32 = 3.0;
    const COLLISION_CHECK_DISTANCE: f32 = 80.0;

    pub struct RivalCart {
        position: Point,
        velocity: Velocity,
        direction: CarDirection,
        distance: f32, // Rival cart's own distance counter
        walls: Vec<Wall>,
    }

    impl RivalCart {
        pub fn new(_walls: &[Wall], _position: Point) -> Self {
            RivalCart {
                position: _position,
                velocity: Velocity {
                    x: 0.0,
                    y: RIVAL_VELOCITY,
                },
                direction: CarDirection::Normal,
                distance: 0.0,
                walls: _walls.to_vec(),
            }
        }
        pub fn update(&mut self, _velocity: Velocity) {
    // Update rival's own distance independently
    self.distance += self.velocity.y;
    
    // Update Y position relative to player
    self.position.y += self.velocity.y - _velocity.y;

    // Check for upcoming walls and adjust path using rival's own distance
    self.check_collision_and_adjust(self.distance);

    // Update X position with calculated horizontal velocity
    self.position.x += self.velocity.x;
    
    // Keep rival car within track boundaries
    if self.position.x < 120.0 {
        self.position.x = 120.0;
        self.velocity.x = 0.0;
    } else if self.position.x > 680.0 {
        self.position.x = 680.0;
        self.velocity.x = 0.0;
    }
}

        fn check_collision_and_adjust(&mut self, distance: f32) {
    let check_ahead = distance + COLLISION_CHECK_DISTANCE;

    // Find walls that we might collide with
    let mut safe_x_positions = vec![];
    let mut blocked_ranges = vec![];

    for wall in &self.walls {
        let wall_min_y = wall.p().y.min(wall.q().y);
        let wall_max_y = wall.p().y.max(wall.q().y);

        // Check if wall is ahead of us
        if wall_min_y <= check_ahead && wall_max_y >= distance {
            let wall_x_min = wall.p().x.min(wall.q().x) - RIVAL_CART_WIDTH;
            let wall_x_max = wall.p().x.max(wall.q().x) + RIVAL_CART_WIDTH;
            blocked_ranges.push((wall_x_min, wall_x_max));
        }
    }

    // Find safe x positions (gaps between walls)
    blocked_ranges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut current_safe_start: f32 = 120.0; // Left boundary of track
    let track_right = 680.0; // Right boundary of track

    for (block_start, block_end) in blocked_ranges {
        if current_safe_start < block_start {
            let safe_center = (current_safe_start + block_start) / 2.0;
            if safe_center >= 120.0 && safe_center <= track_right {
                safe_x_positions.push(safe_center);
            }
        }
        current_safe_start = current_safe_start.max(block_end);
    }

    // Add final safe position after last wall
    if current_safe_start < track_right {
        let safe_center = (current_safe_start + track_right) / 2.0;
        safe_x_positions.push(safe_center);
    }

    // Choose best safe position and move towards it
    if !safe_x_positions.is_empty() {
        // Find the closest safe position to current position
        let mut best_safe_x = safe_x_positions[0];
        let mut min_distance = (safe_x_positions[0] - self.position.x).abs();

        for &safe_x in &safe_x_positions {
            let dist = (safe_x - self.position.x).abs();
            if dist < min_distance {
                min_distance = dist;
                best_safe_x = safe_x;
            }
        }

        // Move towards the best safe position
        let move_speed: f32 = 2.0; // Horizontal movement speed
        let distance_to_safe = best_safe_x - self.position.x;
        
        if distance_to_safe.abs() > 5.0 { // Only move if not already close enough
            if distance_to_safe > 0.0 {
                self.velocity.x = move_speed.min(distance_to_safe);
                self.direction = CarDirection::Right;
            } else {
                self.velocity.x = (-move_speed).max(distance_to_safe);
                self.direction = CarDirection::Left;
            }
        } else {
            self.velocity.x = 0.0;
            self.direction = CarDirection::Normal;
        }
    } else {
        // No safe position found, try to move to center of track
        let track_center = (120.0 + 680.0) / 2.0;
        let distance_to_center = track_center - self.position.x;
        let move_speed: f32 = 2.0;
        
        if distance_to_center.abs() > 5.0 {
            if distance_to_center > 0.0 {
                self.velocity.x = move_speed.min(distance_to_center);
                self.direction = CarDirection::Right;
            } else {
                self.velocity.x = (-move_speed).max(distance_to_center);
                self.direction = CarDirection::Left;
            }
        } else {
            self.velocity.x = 0.0;
            self.direction = CarDirection::Normal;
        }
    }
}

        pub fn get_position(&self) -> Point {
            self.position
        }

        pub fn reset(&mut self, position: Point) {
            self.position = position;
            self.velocity = Velocity {
                x: 0.0,
                y: RIVAL_VELOCITY,
            };
            self.direction = CarDirection::Normal;
            self.distance = 0.0;
        }

        pub fn draw(&self, renderer: &Renderer) {
            // Calculate draw position based on rival's position relative to player
            let _draw_position: Point = self.position;
            // Only draw if rival is visible on screen
            //if draw_position_y > -100.0 && draw_position.y < 1100.0 {
            match self.direction {
                CarDirection::Left => {
                    renderer.draw_left_facing_racing_car(&_draw_position);
                }
                CarDirection::Right => {
                    renderer.draw_right_facing_racing_car(&_draw_position);
                }
                CarDirection::Normal => {
                    renderer.draw_normal_racing_car(&_draw_position);
                }
            }
            //}
        }
    }
}

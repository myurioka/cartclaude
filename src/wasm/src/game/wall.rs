#[allow(clippy::all)]
pub mod wall {
    //! wall related functions.
    use crate::engine::{Point, Velocity};
    use crate::game::{Piece, Renderer, State, StateMachine};

    /// Wall represents a line segment boundary in the game world.
    /// Walls are static geometric elements that define collision boundaries.
    #[derive(Clone, Copy)]
    pub struct Wall {
        pub state_machine: StateMachine,
    }
    impl Piece for Wall {
        /// Creates a new Wall instance with specified endpoints and velocity.
        ///
        /// # Arguments
        /// * `p` - Starting point of the wall line segment
        /// * `q` - Ending point of the wall line segment
        /// * `velocity` - Velocity vector for the wall (typically zero for static walls)
        ///
        /// # Returns
        /// A new Wall instance with a running state machine
        fn new(p: Point, q: Point, velocity: Velocity) -> Self {
            Wall {
                state_machine: StateMachine::Running(State::new(p, q, velocity)),
            }
        }
        /// Gets the current state machine of the wall.
        ///
        /// # Arguments
        /// * `&self` - Reference to the wall instance
        ///
        /// # Returns
        /// A copy of the wall's current state machine
        fn get_state_machine(&self) -> StateMachine {
            self.state_machine
        }
        /// Updates the wall's state machine with a new state.
        ///
        /// # Arguments
        /// * `&mut self` - Mutable reference to the wall instance
        /// * `_state_machine` - New state machine to set (will be updated before assignment)
        ///
        /// # Returns
        /// Nothing (unit type)
        fn set_state_machine(&mut self, _state_machine: StateMachine) {
            self.state_machine = _state_machine.update();
        }

        /// Renders the wall as a line segment using the provided renderer.
        ///
        /// # Arguments
        /// * `&self` - Reference to the wall instance
        /// * `renderer` - Reference to the renderer for drawing operations
        ///
        /// # Returns
        /// Nothing (unit type)
        fn draw(&self, renderer: &Renderer) {
            renderer.line(
                &Point {
                    x: self.state_machine.context().p.x,
                    y: self.state_machine.context().p.y,
                },
                &Point {
                    x: self.state_machine.context().q.x,
                    y: self.state_machine.context().q.y,
                },
            );
        }
    }

    /// Static wall data defining the game level geometry.
    /// Each tuple represents (x1, y1, x2, y2) coordinates for wall line segments.
    /// The walls form the boundaries and obstacles of the game world.
    pub const WALLS_DATA: [(f32, f32, f32, f32); 32] = [
        // left wall
        (100.0, 0.0, 100.0, 1000.0),
        (100.0, 1000.0, 200.0, 1400.0),
        (200.0, 1400.0, 50.0, 2200.0),
        (50.0, 2200.0, 400.0, 2900.0),
        (400.0, 2900.0, 400.0, 3700.0),
        (400.0, 3700.0, 100.0, 3700.0),
        (100.0, 3700.0, 100.0, 6900.0),
        (100.0, 6900.0, 200.0, 7200.0),
        (200.0, 7200.0, 100.0, 7600.0),
        (100.0, 7600.0, 100.0, 9900.0),
        // right wall
        (700.0, 0.0, 700.0, 1000.0),
        (700.0, 1000.0, 350.0, 2200.0),
        (350.0, 2200.0, 700.0, 2900.0),
        (700.0, 2900.0, 700.0, 3700.0),
        (700.0, 3700.0, 700.0, 3700.0),
        (700.0, 3700.0, 700.0, 6900.0),
        (700.0, 6900.0, 600.0, 7200.0),
        (600.0, 7200.0, 700.0, 7600.0),
        (700.0, 7600.0, 700.0, 9900.0),
        // left wall in island1
        (450.0, 4600.0, 250.0, 5200.0),
        (250.0, 5200.0, 550.0, 5900.0),
        // right wall in island1
        (450.0, 4600.0, 600.0, 5200.0),
        (600.0, 5200.0, 550.0, 5900.0),
        // left wall in island2
        (200.0, 5400.0, 200.0, 6550.0),
        (200.0, 6550.0, 550.0, 6550.0),
        // right wall in island2
        (200.0, 5400.0, 550.0, 6550.0),
        // left wall in island3
        (400.0, 7400.0, 250.0, 7800.0),
        (250.0, 7800.0, 250.0, 8200.0),
        (250.0, 8200.0, 400.0, 8600.0),
        // right wall in island3
        (400.0, 7400.0, 550.0, 7800.0),
        (550.0, 7800.0, 550.0, 8200.0),
        (550.0, 8200.0, 400.0, 8600.0),
    ];
}

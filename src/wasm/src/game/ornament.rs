#[allow(clippy::all)]
pub mod ornament {
    //! ornament related functions.
    use crate::engine::{Point, Velocity};
    use crate::game::{Piece, Renderer, State, StateMachine};

    /* <-- CONSTANT VALUE */
    const TREE: [&str; 4] = [" $ ", " $$ ", "$$$", " ▯ "];
    const TREE_DISTANCE: f32 = 12.0;
    const FONT_COLOR: &str = "green";
    const GOAL_X: f32 = 100.0;
    const GOAL_Y: f32 = 7450.0;
    const GOAL: [&str; 2] = [
        "□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■",
        "□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■",
    ];
    const GOAL_DISTANCE: f32 = 25.0;

    pub struct Ornament {
        pub state_machine: StateMachine,
    }
    impl Piece for Ornament {
        fn new(p: Point, q: Point, velocity: Velocity) -> Self {
            Ornament {
                state_machine: StateMachine::Running(State::new(p, q, velocity)),
            }
        }
        fn get_state_machine(&self) -> StateMachine {
            self.state_machine
        }
        fn set_state_machine(&mut self, _state_machine: StateMachine) {
            self.state_machine = _state_machine.update();
        }

        fn draw(&self, renderer: &Renderer) {
            // GOAL
            let mut _distance: f32 = 0.0;
            for _goal in &GOAL {
                renderer.text(
                    &Point {
                        x: GOAL_X + self.state_machine.context().p.x,
                        y: GOAL_Y + self.state_machine.context().p.y + _distance,
                    },
                    _goal,
                    FONT_COLOR,
                    "32 myfont",
                    "center",
                );
                _distance += GOAL_DISTANCE;
            }

            // Fruit trees with different fruits
            let trees = [
                (30.0, 100.0, "apple"),      // Red apple tree
                (400.0, 500.0, "orange"),    // Orange tree
                (400.0, 1000.0, "cherry"),   // Cherry tree
                (120.0, 1500.0, "lemon"),    // Lemon tree
                (620.0, 2000.0, "plum"),     // Plum tree
                (240.0, 2300.0, "apple"),    // Another apple tree
                (620.0, 3200.0, "orange"),   // Another orange tree
                (400.0, 4200.0, "cherry"),   // Another cherry tree
                (320.0, 5000.0, "lemon"),    // Another lemon tree
                (-50.0, 6000.0, "plum"),     // Another plum tree
            ];

            // Draw fruit trees using Canvas instead of ASCII
            for tree in trees.iter() {
                renderer.draw_fruit_tree(
                    &Point {
                        x: tree.0 + self.state_machine.context().p.x,
                        y: tree.1 + self.state_machine.context().p.y,
                    },
                    tree.2, // fruit type
                );
            }
        }
    }
}

#[allow(clippy::all)]
pub mod cart {
    //! Cart module summarizes cart related functions.

    use crate::engine::{Line, Point, Renderer, Velocity};
    use crate::game::CART_START_Y;

    /// Cart width used for collision detection and positioning
    pub const CART_WIDTH: f32 = 20.0;

    /// Cart height used for collision detection and positioning
    pub const CART_HEIGHT: f32 = 50.0;

    ///
    /// A cart entity in the game system.
    ///
    /// The `Cart` struct manages cart behavior using a state machine pattern.
    /// It can be in one of three states: `Idle`(stationary), `Running`(moving),
    /// or `Knocked`(after collision).
    ///
    pub struct Cart {
        pub state_machine: CartStateMachine,
    }
    impl Cart {
        ///
        /// Creates a cart with the given position, velocity, width, height
        ///
        /// # Arguments
        /// * `position`: The Cart's initial position
        /// * `velocity`: The Cart's initial velocity
        /// * `width`: The Cart's width is used for hit detection
        /// * `height`: The Cart's height is used for hit detection
        ///
        /// # Returns
        /// A new Cart instance with the specified parameters
        pub fn new(position: Point, velocity: Velocity) -> Self {
            Cart {
                state_machine: CartStateMachine::Idle(CartState::new(position, velocity)),
            }
        }
        ///
        /// Get a clone of the current cart's state machine
        ///
        /// This method returns a clone of the cart's current state machine,
        /// allowing access to the cart's state without borrowing.
        ///
        /// # Returns
        /// A cloned `CartStateMachine` representing the current state
        fn get_state_machine(&self) -> CartStateMachine {
            self.state_machine.clone()
        }
        ///
        /// Updates the cart's state machine with a new state
        ///
        /// This method takes a state machine, applies an update to it,
        /// and sets it as the cart's current state machine.
        ///
        /// # Arguments
        /// * `_state_machine` - The new `CartStateMachine` to set after updating
        fn set_state_machine(&mut self, _state_machine: CartStateMachine) {
            self.state_machine = _state_machine.update();
        }
        ///
        /// Updates the cart's state machine
        ///
        /// Triggers an update cycle on the current state machine,
        /// allowing state-specific update logic to be executed.
        pub fn update(&mut self) {
            let _state_machine = self.get_state_machine();
            self.set_state_machine(_state_machine);
        }

        ///
        /// Initiates cart movement with the specified velocity
        ///
        /// Transitions the cart to running state and applies the given velocity.
        /// Can be called on both idle and running carts to change direction/speed.
        ///
        /// # Arguments
        /// * `velocity` - The velocity vector to apply to the cart
        pub fn run(&mut self, velocity: Velocity) {
            let _from_state_machine = self.get_state_machine();
            let _to_state_machine = _from_state_machine.transition(Event::Run(velocity));
            self.set_state_machine(_to_state_machine);
        }

        ///
        /// Marks the cart as knocked (hit by collision)
        ///
        /// Transitions the cart from running state to knocked state,
        /// changing its visual appearance and behavior.
        pub fn knocked(&mut self) {
            self.state_machine = self.state_machine.clone().transition(Event::Knocked);
        }

        ///
        /// Checks if the cart intersects with a wall line
        ///
        /// Performs collision detection by checking if any of the cart's boundary lines
        /// intersect with the given wall line using line-to-line intersection algorithm.
        ///
        /// # Arguments
        /// * `_wall_line` - The wall line to check intersection against
        ///
        /// # Returns
        /// * `true` - The cart is intersecting/crossing the wall line
        /// * `false` - The cart is not intersecting the wall line
        pub fn intersect(&self, _wall_line: Line) -> bool {
            let mut _cart_lines = vec![];

            // upper side
            _cart_lines.push(Line::new(
                Point::new(
                    self.state_machine.context().position.x - CART_WIDTH,
                    self.state_machine.context().position.y,
                ),
                Point::new(
                    self.state_machine.context().position.x + CART_WIDTH,
                    self.state_machine.context().position.y,
                ),
            ));
            // right line
            _cart_lines.push(Line::new(
                Point::new(
                    self.state_machine.context().position.x + CART_WIDTH,
                    self.state_machine.context().position.y,
                ),
                Point::new(
                    self.state_machine.context().position.x + CART_WIDTH,
                    self.state_machine.context().position.y + CART_HEIGHT,
                ),
            ));
            // left line
            _cart_lines.push(Line::new(
                Point::new(
                    self.state_machine.context().position.x - CART_WIDTH,
                    self.state_machine.context().position.y - CART_HEIGHT,
                ),
                Point::new(
                    self.state_machine.context().position.x - CART_WIDTH,
                    self.state_machine.context().position.y,
                ),
            ));

            let mut _ans: bool = false;

            for _cart_line in &_cart_lines {
                if ((_cart_line.p.x - _cart_line.q.x) * (_wall_line.p.y - _cart_line.p.y)
                    + (_cart_line.p.y - _cart_line.q.y) * (_cart_line.p.x - _wall_line.p.x))
                    * ((_cart_line.p.x - _cart_line.q.x) * (_wall_line.q.y - _cart_line.p.y)
                        + (_cart_line.p.y - _cart_line.q.y) * (_cart_line.p.x - _wall_line.q.x))
                    < 0.0
                    && ((_wall_line.p.x - _wall_line.q.x) * (_cart_line.p.y - _wall_line.p.y)
                        + (_wall_line.p.y - _wall_line.q.y) * (_wall_line.p.x - _cart_line.p.x))
                        * ((_wall_line.p.x - _wall_line.q.x) * (_cart_line.q.y - _wall_line.p.y)
                            + (_wall_line.p.y - _wall_line.q.y) * (_wall_line.p.x - _cart_line.q.x))
                        < 0.0
                {
                    _ans = true;
                    break;
                }
            }
            return _ans;
        }
        ///
        /// Gets the current position of the cart
        ///
        /// # Returns
        /// The current `Point` position of the cart
        pub fn get_position(&self) -> Point {
            self.state_machine.context().position
        }

        ///
        /// Gets the current velocity of the cart
        ///
        /// # Returns
        /// The current `Velocity` of the cart
        pub fn get_velocity(&self) -> Velocity {
            self.state_machine.context().velocity
        }

        ///
        /// Sets the cart's direction explicitly
        ///
        /// # Arguments
        /// * `direction` - The new direction for the cart
        pub fn set_direction(&mut self, direction: CarDirection) {
            let current_context = self.get_state_machine().context().clone();
            let updated_context = CartContext {
                position: current_context.position,
                velocity: current_context.velocity,
                direction,
            };

            // Update the context in the state machine
            match &mut self.state_machine {
                CartStateMachine::Idle(state) => {
                    state.context = updated_context;
                }
                CartStateMachine::Running(state) => {
                    state.context = updated_context;
                }
                CartStateMachine::Knocked(state) => {
                    state.context = updated_context;
                }
            }
        }

        pub fn draw(&self, renderer: &Renderer) {
            let state_machine = self.get_state_machine();
            let context = state_machine.context();
            let position = &Point {
                x: context.position.x,
                y: CART_START_Y, // ベース位置を使用
            };

            // 状態に応じてCanvas描画メソッドを使用
            match &self.state_machine {
                CartStateMachine::Knocked(_) => {
                    // ダメージ状態：o❚o / ◆ / O❚ O
                    renderer.draw_knocked_racing_car(position);
                }
                _ => {
                    // 通常状態または実行中：向きに応じて描画
                    match context.direction {
                        CarDirection::Left => {
                            renderer.draw_left_facing_racing_car(position);
                        }
                        CarDirection::Right => {
                            renderer.draw_right_facing_racing_car(position);
                        }
                        CarDirection::Normal => {
                            renderer.draw_normal_racing_car(position);
                        }
                    }
                }
            }
        }
    }

    ///
    /// State machine for managing cart behavior
    ///
    /// This enum represents the different states a cart can be in during gameplay.
    /// The state machine handles transitions between states based on events and
    /// maintains the cart's context (position, velocity, dimensions).
    ///
    /// # States
    /// * `Idle` - Cart is stationary and not moving
    /// * `Running` - Cart is actively moving with velocity
    /// * `Knocked` - Cart has been hit and is in a collision state
    #[derive(Clone)]
    pub enum CartStateMachine {
        /// Cart is stationary and waiting for input
        Idle(CartState<Idle>),
        /// Cart is actively moving with velocity
        Running(CartState<Running>),
        /// Cart has been knocked by collision
        Knocked(CartState<Knocked>),
    }

    /// Events that can trigger state transitions in the cart's state machine.
    /// These events represent actions or occurrences that cause the cart to change state.
    pub enum Event {
        /// Start or change the cart's movement with the specified velocity
        Run(Velocity),
        /// Update the cart's state (called each frame)
        Update,
        /// Cart has been knocked/hit by collision
        Knocked,
    }

    impl CartStateMachine {
        /// Handles state transitions based on events.
        ///
        /// # Arguments
        /// * `self` - Current state machine instance (consumed)
        /// * `event` - Event that triggers the transition
        ///
        /// # Returns
        /// New state machine after processing the event
        fn transition(self, event: Event) -> Self {
            match (self.clone(), event) {
                (CartStateMachine::Idle(state), Event::Update) => state.update().into(),
                (CartStateMachine::Idle(state), Event::Run(velocity)) => state.run(velocity).into(),
                (CartStateMachine::Running(state), Event::Run(velocity)) => {
                    state.run(velocity).into()
                }
                (CartStateMachine::Running(state), Event::Update) => state.update().into(),
                (CartStateMachine::Running(state), Event::Knocked) => state.knocked().into(),
                _ => self,
            }
        }
        /// Gets a reference to the cart's context data.
        ///
        /// # Arguments
        /// * `&self` - Reference to the state machine
        ///
        /// # Returns
        /// Reference to the CartContext containing position, velocity, and dimensions
        pub fn context(&self) -> &CartContext {
            match self {
                CartStateMachine::Idle(state) => state.context(),
                CartStateMachine::Running(state) => state.context(),
                CartStateMachine::Knocked(state) => state.context(),
            }
        }
        /// Updates the state machine by triggering an Update event.
        ///
        /// # Arguments
        /// * `self` - State machine instance (consumed)
        ///
        /// # Returns
        /// Updated state machine after processing the Update event
        fn update(self) -> Self {
            self.transition(Event::Update)
        }
    }
    /// Converts an Idle cart state into a CartStateMachine.
    /// This allows seamless conversion from specific state types to the state machine enum.
    impl From<CartState<Idle>> for CartStateMachine {
        /// Converts CartState<Idle> into CartStateMachine::Idle variant.
        ///
        /// # Arguments
        /// * `state` - The idle cart state to convert
        ///
        /// # Returns
        /// CartStateMachine::Idle containing the provided state
        fn from(state: CartState<Idle>) -> Self {
            CartStateMachine::Idle(state)
        }
    }
    /// Converts a Running cart state into a CartStateMachine.
    /// This allows seamless conversion from specific state types to the state machine enum.
    impl From<CartState<Running>> for CartStateMachine {
        /// Converts CartState<Running> into CartStateMachine::Running variant.
        ///
        /// # Arguments
        /// * `state` - The running cart state to convert
        ///
        /// # Returns
        /// CartStateMachine::Running containing the provided state
        fn from(state: CartState<Running>) -> Self {
            CartStateMachine::Running(state)
        }
    }
    /// Converts a Knocked cart state into a CartStateMachine.
    /// This allows seamless conversion from specific state types to the state machine enum.
    impl From<CartState<Knocked>> for CartStateMachine {
        /// Converts CartState<Knocked> into CartStateMachine::Knocked variant.
        ///
        /// # Arguments
        /// * `state` - The knocked cart state to convert
        ///
        /// # Returns
        /// CartStateMachine::Knocked containing the provided state
        fn from(state: CartState<Knocked>) -> Self {
            CartStateMachine::Knocked(state)
        }
    }
    #[derive(Clone)]
    pub struct CartState<S> {
        context: CartContext,
        _state: S,
    }
    impl<S> CartState<S> {
        pub fn context(&self) -> &CartContext {
            &self.context
        }
        fn update_context(&mut self) {
            self.context = self.context.clone().update();
        }
    }

    #[derive(Copy, Clone)]
    pub struct Idle;
    impl CartState<Idle> {
        /// Creates a new idle cart state with the specified parameters.
        ///
        /// # Arguments
        /// * `position` - Initial position of the cart
        /// * `velocity` - Initial velocity of the cart (typically zero for idle state)
        /// * `width` - Width of the cart for collision detection
        /// * `height` - Height of the cart for collision detection
        ///
        /// # Returns
        /// A new CartState<Idle> instance
        pub fn new(position: Point, velocity: Velocity) -> Self {
            CartState {
                context: CartContext::new(position, velocity),
                _state: Idle {},
            }
        }
        /// Updates the idle cart state.
        /// Currently performs context update without changing position since cart is idle.
        ///
        /// # Arguments
        /// * `self` - The idle cart state (consumed)
        ///
        /// # Returns
        /// Updated CartState<Idle> after context update
        pub fn update(mut self) -> CartState<Idle> {
            self.update_context();
            self
        }
        /// Transitions from idle to running state with the specified velocity.
        ///
        /// # Arguments
        /// * `self` - The idle cart state (consumed)
        /// * `velocity` - The velocity to apply when transitioning to running state
        ///
        /// # Returns
        /// A new CartState<Running> with the applied velocity
        pub fn run(self, velocity: Velocity) -> CartState<Running> {
            CartState {
                context: self.context.run(velocity),
                _state: Running {},
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct Running;
    impl CartState<Running> {
        /// Updates the running cart state by applying velocity to position.
        /// Moves the cart horizontally based on its current velocity.
        ///
        /// # Arguments
        /// * `self` - The running cart state (consumed)
        ///
        /// # Returns
        /// Updated CartState<Running> with new position
        pub fn update(mut self) -> CartState<Running> {
            self.context.position.x += self.context.velocity.x;
            self.update_context();
            self
        }
        /// Changes the velocity of the running cart while maintaining running state.
        ///
        /// # Arguments
        /// * `self` - The running cart state (consumed)
        /// * `velocity` - The new velocity to apply to the cart
        ///
        /// # Returns
        /// CartState<Running> with updated velocity
        pub fn run(self, velocity: Velocity) -> CartState<Running> {
            CartState {
                context: self.context.run(velocity),
                _state: Running {},
            }
        }
        /// Transitions from running to knocked state after collision.
        ///
        /// # Arguments
        /// * `self` - The running cart state (consumed)
        ///
        /// # Returns
        /// CartState<Knocked> representing the cart after being hit
        pub fn knocked(self) -> CartState<Knocked> {
            CartState {
                context: self.context.knocked(),
                _state: Knocked {},
            }
        }
    }

    /// Knocked state marker - represents a cart that has been hit by collision.
    /// In the knocked state, the cart cannot perform actions and remains stationary.
    #[derive(Copy, Clone)]
    pub struct Knocked;

    /// Implementation for CartState<Knocked>.
    /// Currently, knocked carts have no available actions and remain in this state.
    impl CartState<Knocked> {}

    #[derive(Clone)]
    pub struct CartContext {
        position: Point,
        velocity: Velocity,
        direction: CarDirection, // 追加：車の向き
    }

    /// 車の向きを表す列挙型
    #[derive(Clone, Copy, Debug)]
    pub enum CarDirection {
        Normal, // 通常（正面）
        Left,   // 左向き
        Right,  // 右向き
    }

    impl CartContext {
        fn new(_position: Point, _velocity: Velocity) -> CartContext {
            CartContext {
                position: _position,
                velocity: _velocity,
                direction: CarDirection::Normal, // デフォルトは通常状態
            }
        }
        fn update(self) -> Self {
            self
        }
        fn run(mut self, velocity: Velocity) -> Self {
            self.velocity = velocity;
            // 速度に応じて向きを自動設定
            let direction = self.direction;
            self.set_direction(direction)
        }
        fn knocked(self) -> Self {
            self
        }

        /// 車の向きを設定
        fn set_direction(mut self, direction: CarDirection) -> Self {
            self.direction = direction;
            // 速度に応じて自動的に向きを設定
            if self.velocity.x < -0.1 {
                self.direction = CarDirection::Left;
            } else if self.velocity.x > 0.1 {
                self.direction = CarDirection::Right;
            }
            self
        }
    }
}

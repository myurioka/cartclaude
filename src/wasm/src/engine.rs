use crate::browser::{self, LoopClosure};
use crate::game::CANVAS_HEIGHT;
use crate::sound;
//use num_traits::FromPrimitive;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use futures::channel::{
    mpsc::{UnboundedReceiver, unbounded},
    //oneshot::channel,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::{AudioBuffer, AudioContext};

pub const FONT_COLOR: &str = "green";

#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl Point {
    pub fn new(_x: f32, _y: f32) -> Point {
        Point { x: _x, y: _y }
    }
    pub fn add(&self, _v: Velocity) -> Point {
        Point {
            x: self.x - _v.x,
            y: self.y - _v.y,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
pub struct Line {
    pub p: Point,
    pub q: Point,
}
impl Line {
    pub fn new(_p: Point, _q: Point) -> Line {
        Line { p: _p, q: _q }
    }
}

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    /// Draw a small filled wheel (o) at the specified position
    pub fn draw_small_wheel(&self, point: &Point, color: &str) {
        self.context.set_fill_style_str(color);
        self.context.begin_path();
        self.context
            .arc(
                point.x as f64,
                CANVAS_HEIGHT as f64 - point.y as f64,
                3.0, // radius
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap_or(());
        self.context.fill();

        // Add stroke for better visibility
        self.context.set_stroke_style_str("#1a3f2a");
        self.context.set_line_width(0.5);
        self.context.stroke();
    }

    /// Draw a large hollow wheel (O) at the specified position
    pub fn draw_large_wheel(&self, point: &Point, color: &str) {
        self.context.set_stroke_style_str(color);
        self.context.set_line_width(2.0);
        self.context.begin_path();
        self.context
            .arc(
                point.x as f64,
                CANVAS_HEIGHT as f64 - point.y as f64,
                4.0, // radius
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap_or(());
        self.context.stroke();
    }

    /// Draw a center body (●) at the specified position
    pub fn draw_center_body(&self, point: &Point, color: &str) {
        self.context.set_fill_style_str(color);
        self.context.begin_path();
        self.context
            .arc(
                point.x as f64,
                CANVAS_HEIGHT as f64 - point.y as f64,
                2.5, // radius
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap_or(());
        self.context.fill();

        // Add stroke for better definition
        self.context.set_stroke_style_str("#aa2222");
        self.context.set_line_width(0.5);
        self.context.stroke();
    }

    /// Draw a diamond body (◆) at the specified position
    pub fn draw_diamond_body(&self, point: &Point, color: &str) {
        let canvas_y = CANVAS_HEIGHT as f64 - point.y as f64;

        self.context.set_fill_style_str(color);
        self.context.begin_path();

        // Diamond shape: top, right, bottom, left
        self.context.move_to(point.x as f64, canvas_y - 8.0); // top
        self.context.line_to(point.x as f64 + 8.0, canvas_y); // right
        self.context.line_to(point.x as f64, canvas_y + 8.0); // bottom
        self.context.line_to(point.x as f64 - 8.0, canvas_y); // left
        self.context.close_path();
        self.context.fill();

        // Add stroke
        self.context.set_stroke_style_str("#2a5f41");
        self.context.set_line_width(1.0);
        self.context.stroke();
    }

    /// Draw racing car in normal state (o●o / ◆ / O●O)
    pub fn draw_normal_racing_car(&self, position: &Point) {
        let wheel_color = "#2a5f41"; // Green wheels
        let body_color = "#cc3333"; // Red body
        let diamond_color = "#4a9f6a"; // Green diamond

        // Row 1: o●o (small wheels and center body)
        let row1_y = position.y;
        self.draw_small_wheel(
            &Point {
                x: position.x - 12.0,
                y: row1_y,
            },
            wheel_color,
        );
        self.draw_center_body(
            &Point {
                x: position.x,
                y: row1_y,
            },
            body_color,
        );
        self.draw_small_wheel(
            &Point {
                x: position.x + 12.0,
                y: row1_y,
            },
            wheel_color,
        );

        // Row 2: ◆ (diamond body)
        let row2_y = position.y - 18.0; // CART_DISTANCE
        self.draw_diamond_body(
            &Point {
                x: position.x,
                y: row2_y,
            },
            diamond_color,
        );

        // Row 3: O●O (large wheels and center body)
        let row3_y = position.y - 36.0; // 2 * CART_DISTANCE
        self.draw_large_wheel(
            &Point {
                x: position.x - 12.0,
                y: row3_y,
            },
            wheel_color,
        );
        self.draw_center_body(
            &Point {
                x: position.x,
                y: row3_y,
            },
            body_color,
        );
        self.draw_large_wheel(
            &Point {
                x: position.x + 12.0,
                y: row3_y,
            },
            wheel_color,
        );
    }

    /// Draw rival racing car in normal state with blue color (o●o / ◆ / O●O)
    pub fn draw_blue_normal_racing_car(&self, position: &Point) {
        let wheel_color = "#1a4f5a"; // Dark blue wheels
        let body_color = "#3366cc"; // Blue body
        let diamond_color = "#4a9f6a"; // Green diamond

        // Row 1: o●o (small wheels and center body)
        let row1_y = position.y;
        self.draw_small_wheel(
            &Point {
                x: position.x - 12.0,
                y: row1_y,
            },
            wheel_color,
        );
        self.draw_center_body(
            &Point {
                x: position.x,
                y: row1_y,
            },
            body_color,
        );
        self.draw_small_wheel(
            &Point {
                x: position.x + 12.0,
                y: row1_y,
            },
            wheel_color,
        );

        // Row 2: ◆ (diamond body)
        let row2_y = position.y - 18.0; // CART_DISTANCE
        self.draw_diamond_body(
            &Point {
                x: position.x,
                y: row2_y,
            },
            diamond_color,
        );

        // Row 3: O●O (large wheels and center body)
        let row3_y = position.y - 36.0; // 2 * CART_DISTANCE
        self.draw_large_wheel(
            &Point {
                x: position.x - 12.0,
                y: row3_y,
            },
            wheel_color,
        );
        self.draw_center_body(
            &Point {
                x: position.x,
                y: row3_y,
            },
            body_color,
        );
        self.draw_large_wheel(
            &Point {
                x: position.x + 12.0,
                y: row3_y,
            },
            wheel_color,
        );
    }

    /// Draw racing car in knocked/damaged state (O● O / ◆ / o ●o)
    pub fn draw_knocked_racing_car(&self, position: &Point) {
        let wheel_color = "#2a5f41"; // Green wheels
        let body_color = "#cc3333"; // Red body
        let diamond_color = "#4a9f6a"; // Green diamond

        // Row 1: O● O (large wheels spread apart with center body)
        let row1_y = position.y;
        self.draw_large_wheel(
            &Point {
                x: position.x - 16.0,
                y: row1_y,
            },
            wheel_color,
        ); // Left further
        self.draw_center_body(
            &Point {
                x: position.x,
                y: row1_y,
            },
            body_color,
        );
        self.draw_large_wheel(
            &Point {
                x: position.x + 16.0,
                y: row1_y,
            },
            wheel_color,
        ); // Right further

        // Row 2: ◆ (diamond body - unchanged)
        let row2_y = position.y - 18.0;
        self.draw_diamond_body(
            &Point {
                x: position.x,
                y: row2_y,
            },
            diamond_color,
        );

        // Row 3: o ●o (small wheels with center body)
        let row3_y = position.y - 36.0;
        self.draw_small_wheel(
            &Point {
                x: position.x - 12.0,
                y: row3_y,
            },
            wheel_color,
        );
        self.draw_center_body(
            &Point {
                x: position.x,
                y: row3_y,
            },
            body_color,
        );
        self.draw_small_wheel(
            &Point {
                x: position.x + 12.0,
                y: row3_y,
            },
            wheel_color,
        );
    }

    /// Draw racing car facing left (with perspective)
    pub fn draw_left_facing_racing_car(&self, position: &Point) {
        let wheel_color = "#2a5f41";
        let body_color = "#cc3333";
        let diamond_color = "#4a9f6a";

        // Row 1: Elliptical wheels for perspective
        let row1_y = position.y;
        self.draw_ellipse_wheel(
            &Point {
                x: position.x - 10.0,
                y: row1_y,
            },
            4.0,
            3.0,
            wheel_color,
            true,
        );
        self.draw_center_body(
            &Point {
                x: position.x + 2.0,
                y: row1_y,
            },
            body_color,
        );
        self.draw_ellipse_wheel(
            &Point {
                x: position.x + 14.0,
                y: row1_y,
            },
            3.0,
            4.0,
            wheel_color,
            true,
        );

        // Row 2: Diamond slightly offset left
        let row2_y = position.y - 18.0;
        self.draw_diamond_body(
            &Point {
                x: position.x - 2.0,
                y: row2_y,
            },
            diamond_color,
        );

        // Row 3: Elliptical large wheels for perspective
        let row3_y = position.y - 36.0;
        self.draw_ellipse_wheel(
            &Point {
                x: position.x - 10.0,
                y: row3_y,
            },
            5.0,
            4.0,
            wheel_color,
            false,
        );
        self.draw_center_body(
            &Point {
                x: position.x + 2.0,
                y: row3_y,
            },
            body_color,
        );
        self.draw_ellipse_wheel(
            &Point {
                x: position.x + 14.0,
                y: row3_y,
            },
            4.0,
            5.0,
            wheel_color,
            false,
        );
    }

    /// Draw blue rival racing car facing left (with perspective)
    pub fn draw_blue_left_facing_racing_car(&self, position: &Point) {
        let wheel_color = "#1a4f5a"; // Dark blue wheels
        let body_color = "#3366cc"; // Blue body
        let diamond_color = "#4a9f6a"; // Green diamond

        // Row 1: Elliptical wheels for perspective
        let row1_y = position.y;
        self.draw_ellipse_wheel(
            &Point {
                x: position.x - 10.0,
                y: row1_y,
            },
            4.0,
            3.0,
            wheel_color,
            true,
        );
        self.draw_center_body(
            &Point {
                x: position.x + 2.0,
                y: row1_y,
            },
            body_color,
        );
        self.draw_ellipse_wheel(
            &Point {
                x: position.x + 14.0,
                y: row1_y,
            },
            3.0,
            4.0,
            wheel_color,
            true,
        );

        // Row 2: Diamond slightly offset left
        let row2_y = position.y - 18.0;
        self.draw_diamond_body(
            &Point {
                x: position.x - 2.0,
                y: row2_y,
            },
            diamond_color,
        );

        // Row 3: Elliptical large wheels for perspective
        let row3_y = position.y - 36.0;
        self.draw_ellipse_wheel(
            &Point {
                x: position.x - 10.0,
                y: row3_y,
            },
            5.0,
            4.0,
            wheel_color,
            false,
        );
        self.draw_center_body(
            &Point {
                x: position.x + 2.0,
                y: row3_y,
            },
            body_color,
        );
        self.draw_ellipse_wheel(
            &Point {
                x: position.x + 14.0,
                y: row3_y,
            },
            4.0,
            5.0,
            wheel_color,
            false,
        );
    }

    /// Draw racing car facing right (with perspective)
    pub fn draw_right_facing_racing_car(&self, position: &Point) {
        let wheel_color = "#2a5f41";
        let body_color = "#cc3333";
        let diamond_color = "#4a9f6a";

        // Row 1: Elliptical wheels for perspective
        let row1_y = position.y;
        self.draw_ellipse_wheel(
            &Point {
                x: position.x - 14.0,
                y: row1_y,
            },
            3.0,
            4.0,
            wheel_color,
            true,
        );
        self.draw_center_body(
            &Point {
                x: position.x - 2.0,
                y: row1_y,
            },
            body_color,
        );
        self.draw_ellipse_wheel(
            &Point {
                x: position.x + 10.0,
                y: row1_y,
            },
            4.0,
            3.0,
            wheel_color,
            true,
        );

        // Row 2: Diamond slightly offset right
        let row2_y = position.y - 18.0;
        self.draw_diamond_body(
            &Point {
                x: position.x + 2.0,
                y: row2_y,
            },
            diamond_color,
        );

        // Row 3: Elliptical large wheels for perspective
        let row3_y = position.y - 36.0;
        self.draw_ellipse_wheel(
            &Point {
                x: position.x - 14.0,
                y: row3_y,
            },
            4.0,
            5.0,
            wheel_color,
            false,
        );
        self.draw_center_body(
            &Point {
                x: position.x - 2.0,
                y: row3_y,
            },
            body_color,
        );
        self.draw_ellipse_wheel(
            &Point {
                x: position.x + 10.0,
                y: row3_y,
            },
            5.0,
            4.0,
            wheel_color,
            false,
        );
    }

    /// Draw blue rival racing car facing right (with perspective)
    pub fn draw_blue_right_facing_racing_car(&self, position: &Point) {
        let wheel_color = "#1a4f5a"; // Dark blue wheels
        let body_color = "#3366cc"; // Blue body
        let diamond_color = "#4a9f6a";

        // Row 1: Elliptical wheels for perspective
        let row1_y = position.y;
        self.draw_ellipse_wheel(
            &Point {
                x: position.x - 14.0,
                y: row1_y,
            },
            3.0,
            4.0,
            wheel_color,
            true,
        );
        self.draw_center_body(
            &Point {
                x: position.x - 2.0,
                y: row1_y,
            },
            body_color,
        );
        self.draw_ellipse_wheel(
            &Point {
                x: position.x + 10.0,
                y: row1_y,
            },
            4.0,
            3.0,
            wheel_color,
            true,
        );

        // Row 2: Diamond slightly offset right
        let row2_y = position.y - 18.0;
        self.draw_diamond_body(
            &Point {
                x: position.x + 2.0,
                y: row2_y,
            },
            diamond_color,
        );

        // Row 3: Elliptical large wheels for perspective
        let row3_y = position.y - 36.0;
        self.draw_ellipse_wheel(
            &Point {
                x: position.x - 14.0,
                y: row3_y,
            },
            4.0,
            5.0,
            wheel_color,
            false,
        );
        self.draw_center_body(
            &Point {
                x: position.x - 2.0,
                y: row3_y,
            },
            body_color,
        );
        self.draw_ellipse_wheel(
            &Point {
                x: position.x + 10.0,
                y: row3_y,
            },
            5.0,
            4.0,
            wheel_color,
            false,
        );
    }

    /// Helper method to draw elliptical wheels for perspective views
    fn draw_ellipse_wheel(
        &self,
        point: &Point,
        radius_x: f64,
        radius_y: f64,
        color: &str,
        filled: bool,
    ) {
        let canvas_y = CANVAS_HEIGHT as f64 - point.y as f64;

        if filled {
            self.context.set_fill_style_str(color);
        }
        self.context.set_stroke_style_str(color);
        self.context.set_line_width(if filled { 0.5 } else { 2.0 });

        self.context.begin_path();
        self.context
            .ellipse(
                point.x as f64,
                canvas_y,
                radius_x,
                radius_y,
                0.0,                        // rotation
                0.0,                        // start angle
                std::f64::consts::PI * 2.0, // end angle
            )
            .unwrap_or(());

        if filled {
            self.context.fill();
        }
        self.context.stroke();
    }

    /// Draw a tree trunk (▯) at the specified position
    pub fn draw_tree_trunk(&self, point: &Point, color: &str) {
        let canvas_y = CANVAS_HEIGHT as f64 - point.y as f64;

        self.context.set_fill_style_str(color);
        self.context.set_stroke_style_str("#8B4513"); // Brown stroke
        self.context.set_line_width(1.0);

        self.context.begin_path();
        // Rectangle trunk: width=8, height=12
        self.context.rect(
            point.x as f64 - 4.0, // x - width/2
            canvas_y - 6.0,       // y - height/2
            8.0,                  // width
            12.0,                 // height
        );
        self.context.fill();
        self.context.stroke();
    }

    /// Draw tree leaves ($) as a circle at the specified position
    pub fn draw_tree_leaves(&self, point: &Point, color: &str, radius: f32) {
        self.context.set_fill_style_str(color);
        self.context.set_stroke_style_str("#2d5016"); // Dark green stroke
        self.context.set_line_width(0.5);

        self.context.begin_path();
        self.context
            .arc(
                point.x as f64,
                CANVAS_HEIGHT as f64 - point.y as f64,
                radius as f64,
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap_or(());
        self.context.fill();
        self.context.stroke();
    }

    /// Draw a fruit on tree (various colored circles for different fruits)
    pub fn draw_fruit(&self, point: &Point, fruit_type: &str) {
        let (color, stroke_color, radius) = match fruit_type {
            "apple" => ("#ff4444", "#cc2222", 3.0),  // Red apple
            "orange" => ("#ff8800", "#dd6600", 3.5), // Orange
            "cherry" => ("#dd0000", "#aa0000", 2.5), // Red cherry
            "lemon" => ("#ffff44", "#dddd22", 3.0),  // Yellow lemon
            "plum" => ("#8844ff", "#6622dd", 3.0),   // Purple plum
            _ => ("#44ff44", "#22dd22", 3.0),        // Default green
        };

        self.context.set_fill_style_str(color);
        self.context.set_stroke_style_str(stroke_color);
        self.context.set_line_width(0.8);

        self.context.begin_path();
        self.context
            .arc(
                point.x as f64,
                CANVAS_HEIGHT as f64 - point.y as f64,
                radius,
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap_or(());
        self.context.fill();
        self.context.stroke();
    }

    /// Draw a complete fruit tree at the specified position
    pub fn draw_fruit_tree(&self, position: &Point, fruit_type: &str) {
        let tree_distance = 12.0;

        // Layer 1: " $ " - Single small leaf cluster
        self.draw_tree_leaves(
            &Point {
                x: position.x,
                y: position.y,
            },
            "#228b22", // Forest green
            6.0,
        );
        // Add a fruit to the top
        self.draw_fruit(
            &Point {
                x: position.x + 4.0,
                y: position.y + 2.0,
            },
            fruit_type,
        );

        // Layer 2: " $$ " - Two leaf clusters
        let layer2_y = position.y - tree_distance;
        self.draw_tree_leaves(
            &Point {
                x: position.x - 6.0,
                y: layer2_y,
            },
            "#32cd32", // Lime green
            7.0,
        );
        self.draw_tree_leaves(
            &Point {
                x: position.x + 6.0,
                y: layer2_y,
            },
            "#32cd32",
            7.0,
        );
        // Add fruits to both clusters
        self.draw_fruit(
            &Point {
                x: position.x - 3.0,
                y: layer2_y + 3.0,
            },
            fruit_type,
        );
        self.draw_fruit(
            &Point {
                x: position.x + 9.0,
                y: layer2_y - 2.0,
            },
            fruit_type,
        );

        // Layer 3: "$$$" - Three leaf clusters (main canopy)
        let layer3_y = position.y - tree_distance * 2.0;
        self.draw_tree_leaves(
            &Point {
                x: position.x - 12.0,
                y: layer3_y,
            },
            "#228b22", // Forest green
            8.0,
        );
        self.draw_tree_leaves(
            &Point {
                x: position.x,
                y: layer3_y,
            },
            "#32cd32", // Lime green (center)
            9.0,
        );
        self.draw_tree_leaves(
            &Point {
                x: position.x + 12.0,
                y: layer3_y,
            },
            "#228b22", // Forest green
            8.0,
        );
        // Add multiple fruits to the main canopy
        self.draw_fruit(
            &Point {
                x: position.x - 8.0,
                y: layer3_y + 4.0,
            },
            fruit_type,
        );
        self.draw_fruit(
            &Point {
                x: position.x + 2.0,
                y: layer3_y - 3.0,
            },
            fruit_type,
        );
        self.draw_fruit(
            &Point {
                x: position.x + 8.0,
                y: layer3_y + 2.0,
            },
            fruit_type,
        );

        // Layer 4: " ▯ " - Tree trunk
        let layer4_y = position.y - tree_distance * 3.0;
        self.draw_tree_trunk(
            &Point {
                x: position.x,
                y: layer4_y,
            },
            "#8b4513", // Saddle brown
        );
    }
}

impl Renderer {
    pub fn clear(&self, point: &Point, width: f32, height: f32) {
        self.context
            .clear_rect(point.x.into(), point.y.into(), width as f64, height as f64);
    }
    pub fn text(&self, point: &Point, text: &str, color: &str, font: &str, align: &str) {
        self.context.set_fill_style_str(color);
        self.context.set_text_align(align);
        self.context.set_text_baseline("middle");
        self.context.set_font(font);
        let _ = self
            .context
            .fill_text(text, point.x as f64, CANVAS_HEIGHT as f64 - point.y as f64);
    }
    pub fn line(&self, p: &Point, q: &Point) {
        self.context.begin_path();
        self.context.set_stroke_style_str(FONT_COLOR);
        self.context
            .move_to(p.x.into(), CANVAS_HEIGHT as f64 - p.y as f64);
        self.context
            .line_to(q.x.into(), CANVAS_HEIGHT as f64 - q.y as f64);
        self.context.close_path();
        self.context.stroke();
    }
}

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self, keystate: &KeyState);
    fn draw(&self, renderer: &Renderer);
}

const FRAME_SIZE: f64 = 1.0 / 60.0 * 1000.0;
pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f64,
}
type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        let mut keyevent_receiver = prepare_input()?;
        let mut game = game.initialize().await?;
        let mut game_loop = GameLoop {
            last_frame: browser::now()?,
            accumulated_delta: 0.0,
        };

        let renderer = Renderer {
            context: browser::context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut keystate = KeyState::new();
        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            process_input(&mut keystate, &mut keyevent_receiver);

            game_loop.accumulated_delta += perf - game_loop.last_frame;
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update(&keystate);
                game_loop.accumulated_delta -= FRAME_SIZE;
            }
            let _last_frame = browser::now().unwrap();
            game.draw(&renderer);
            game_loop.last_frame = _last_frame;

            let _ = browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;
        Ok(())
    }
}

pub struct KeyState {
    pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
}

impl KeyState {
    fn new() -> Self {
        KeyState {
            pressed_keys: HashMap::new(),
        }
    }
    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }

    fn set_pressed(&mut self, code: &str, event: web_sys::KeyboardEvent) {
        self.pressed_keys.insert(code.into(), event);
    }

    fn set_released(&mut self, code: &str) {
        self.pressed_keys.remove(code);
    }
}

enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}

fn process_input(state: &mut KeyState, keyevent_receiver: &mut UnboundedReceiver<KeyPress>) {
    loop {
        match keyevent_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt {
                KeyPress::KeyUp(evt) => state.set_released(&evt.code()),
                KeyPress::KeyDown(evt) => state.set_pressed(&evt.code(), evt),
            },
        };
    }
}

// For Keypress Input
fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);
    let onkeydown = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        let _ = keydown_sender
            .borrow_mut()
            .start_send(KeyPress::KeyDown(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    let onkeyup = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        let _ = keyup_sender
            .borrow_mut()
            .start_send(KeyPress::KeyUp(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    browser::canvas()?.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    browser::canvas()?.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    onkeydown.forget();
    onkeyup.forget();

    Ok(keyevent_receiver)
}

#[derive(Clone)]
pub struct Audio {
    context: AudioContext,
}

#[derive(Clone)]
pub struct Sound {
    pub buffer: AudioBuffer,
}

impl Audio {
    pub fn new() -> Result<Self> {
        Ok(Audio {
            context: sound::create_audio_context()?,
        })
    }

    pub async fn load_sound(&self, filename: &str) -> Result<Sound> {
        let array_buffer = browser::fetch_array_buffer(filename).await?;

        let audio_buffer = sound::decode_audio_data(&self.context, &array_buffer).await?;

        Ok(Sound {
            buffer: audio_buffer,
        })
    }

    pub fn play_sound(&self, sound: &Sound) -> Result<()> {
        sound::play_sound(&self.context, &sound.buffer, sound::Looping::No)
    }

    pub fn play_looping_sound(&self, sound: &Sound) -> Result<()> {
        sound::play_sound(&self.context, &sound.buffer, sound::Looping::Yes)
    }
}

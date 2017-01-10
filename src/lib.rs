//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Simple application framework, similar to the Processing environment
//! used in the book.

extern crate piston_window;
extern crate rand;
extern crate sdl2_window;

pub use math::*;
pub use piston_window::*;
pub use rand::Rng;
pub use types::{Color, ColorComponent};

use sdl2_window::Sdl2Window;

#[derive(Debug)]
pub struct PistonAppState {
    mouse_button: MouseButton,
    mouse_pressed: u8,
    mouse_x: f64,
    mouse_y: f64,
}

impl PistonAppState {
    fn new() -> Self {
        PistonAppState {
            mouse_button: MouseButton::Unknown,
            mouse_pressed: 0,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }

    pub fn mouse_button(&self) -> MouseButton {
        self.mouse_button
    }

    pub fn mouse_pressed(&self) -> bool {
        self.mouse_pressed > 0
    }

    pub fn mouse_x(&self) -> f64 {
        self.mouse_x
    }

    pub fn mouse_y(&self) -> f64 {
        self.mouse_y
    }
}

pub trait PistonApp {
    fn setup(&mut self,
             _context: Context,
             _gl: &mut G2d,
             _state: &PistonAppState,
             _args: &RenderArgs) {
    }

    fn draw(&mut self,
            context: Context,
            gl: &mut G2d,
            state: &PistonAppState,
            args: &RenderArgs);

    fn run<T: Into<String>>(title: T, app: &mut Self) {
        let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(title, [640, 480])
            .exit_on_esc(true)
            .resizable(false)
            .vsync(true)
            .build()
            .unwrap();
        let mut state = PistonAppState::new();
        let mut first = true;
        while let Some(e) = window.next() {
            if let Some(args) = e.render_args() {
                window.draw_2d(&e, |context, gl| {
                    if first {
                        first = false;
                        app.setup(context, gl, &state, &args);
                    }
                    app.draw(context, gl, &state, &args);
                });
            }
            let state = &mut state;
            if let Some(Button::Mouse(_)) = e.press_args() {
                state.mouse_pressed += 1;
            }
            if let Some(Button::Mouse(_)) = e.release_args() {
                if state.mouse_pressed > 0 {
                    state.mouse_pressed -= 1;
                }
            }
            if let Some(position) = e.mouse_cursor_args() {
                state.mouse_x = position[0];
                state.mouse_y = position[1];
            }
        }
    }
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    [rng.gen_range(0.3, 1.0), rng.gen_range(0.3, 1.0), rng.gen_range(0.3, 1.0), 1.0]
}

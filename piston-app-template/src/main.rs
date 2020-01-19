//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Application template.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct App;

impl App {
    fn new() -> Self {
        App {}
    }
}

impl PistonApp for App {
    fn setup(&mut self, _window: &mut PistonAppWindow, _state: &PistonAppState) {
        unimplemented!();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        window.draw_2d(state.event(), |_context, _gfx| {
            unimplemented!();
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

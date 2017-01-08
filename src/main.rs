//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Random walker.

extern crate piston_app;

use piston_app::*;

struct App;

impl App {
    pub fn new() -> Self {
        App {}
    }
}

impl PistonApp for App {
    fn setup(&mut self, context: Context, gl: &mut G2d, args: &RenderArgs) {
        clear([1.0; 4], gl);
    }

    fn draw(&mut self, context: Context, gl: &mut G2d, args: &RenderArgs) {}
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

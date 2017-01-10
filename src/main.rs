//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Gaussian distribution graph.

extern crate piston_app;
extern crate rand;

use piston_app::*;
use rand::distributions::normal::StandardNormal;

#[derive(Debug)]
struct App;

impl App {
    fn new() -> Self {
        App {}
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: Context, gl: &mut G2d, _: &PistonAppState) {
        clear([1.0; 4], gl);
    }

    fn draw(&mut self, context: Context, gl: &mut G2d, state: &PistonAppState) {
        let mean = 320.0;
        let sd = 60.0;
        let StandardNormal(x) = rand::random();
        let result = x * sd + mean;
        ellipse([0.0, 0.0, 0.0, 0.1],
                ellipse::circle(result, state.height() / 2.0, 16.0),
                context.transform,
                gl);
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

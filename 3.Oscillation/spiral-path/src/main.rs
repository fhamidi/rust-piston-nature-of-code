//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Spiral path.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct App {
    r: Scalar,
    theta: Scalar,
    color_offset: Scalar,
}

impl App {
    fn new() -> Self {
        App {
            r: 0.0,
            theta: 0.0,
            color_offset: 0.0,
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        window.draw_2d(state.event(), |_, gfx| {
            clear(color::BLACK, gfx);
        });
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        let (x, y) = (self.r * self.theta.cos(), self.r * self.theta.sin());
        window.draw_2d(state.event(), |context, gfx| {
            ellipse(state.noise_color(self.color_offset, Some(1.0)),
                    ellipse::circle(x + state.width() / 2.0,
                                    y + state.height() / 2.0,
                                    4.0),
                    context.transform,
                    gfx);
        });
        self.r += 0.042;
        self.theta += 0.01;
        self.color_offset += 1e-3;
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

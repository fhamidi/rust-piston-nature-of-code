//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Perlin noise wave.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct App {
    start_angle: Scalar,
    velocity: Scalar,
}

impl App {
    fn new() -> Self {
        App {
            start_angle: 0.0,
            velocity: 0.05,
        }
    }
}

impl PistonApp for App {
    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            let mut angle = self.start_angle;
            self.start_angle += 0.02;
            let mut x = 0.0;
            while x <= state.width() {
                let y =
                    state.map_range(state.noise(&[angle]), 0.0, 1.0, 0.0, state.height());
                Ellipse::new_border(color::BLACK, 1.0)
                    .color([0.0, 0.0, 0.0, 1.0 / 3.0])
                    .draw(ellipse::circle(x, y, 24.0),
                          &context.draw_state,
                          context.transform,
                          gfx);
                angle += self.velocity;
                x += 8.0;
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

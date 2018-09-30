//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Spiral path.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct App {
    base_hue: Scalar,
    color_offset: Scalar,
    r: Scalar,
    theta: Scalar,
}

impl App {
    fn new() -> Self {
        let mut rng = thread_rng();
        App {
            base_hue: rng.gen(),
            color_offset: rng.gen(),
            r: 0.0,
            theta: 0.0,
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
            Ellipse::new(state.noise_color(self.base_hue, self.color_offset, Some(1.0)))
                .resolution(8)
                .draw(
                    ellipse::circle(
                        x + state.width() / 2.0,
                        y + state.height() / 2.0,
                        4.0,
                    ),
                    &context.draw_state,
                    context.transform,
                    gfx,
                );
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

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Simple harmonic motion.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct App {
    bob_color: Color,
    period: Scalar,
    amplitude: Scalar,
}

impl App {
    fn new() -> Self {
        App {
            bob_color: color::TRANSPARENT,
            period: 120.0,
            amplitude: 0.0,
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        self.bob_color = state.random_color(Some(1.0));
        self.amplitude = state.width() / 3.0;
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            let transform = context
                .transform
                .trans(state.width() / 2.0, state.height() / 2.0);
            let frame_count = state.frame_count() as Scalar;
            let x = self.amplitude *
                    (consts::FRAC_2_PI * frame_count / self.period).cos();
            Line::new(color::BLACK, 1.0)
                .draw([0.0, 0.0, x, 0.0], &context.draw_state, transform, gfx);
            Ellipse::new_border(color::BLACK, 1.0)
                .resolution(32)
                .color(self.bob_color)
                .draw(ellipse::circle(x, 0.0, 20.0),
                      &context.draw_state,
                      transform,
                      gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

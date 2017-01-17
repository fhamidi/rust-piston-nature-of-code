//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Vectors - Bouncing ball.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Ball {
    x: Scalar,
    y: Scalar,
    x_speed: Scalar,
    y_speed: Scalar,
}

impl Ball {
    fn new() -> Self {
        Ball {
            x: 128.0,
            y: 128.0,
            x_speed: 2.0,
            y_speed: 10.0 / 3.0,
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .color([0.5, 0.5, 0.5, 1.0])
            .draw(ellipse::circle(self.x, self.y, 32.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn step(&mut self, state: &PistonAppState) {
        self.x += self.x_speed;
        self.y += self.y_speed;
        if self.x > state.width() || self.x < 0.0 {
            self.x_speed *= -1.0;
        }
        if self.y > state.height() || self.y < 0.0 {
            self.y_speed *= -1.0;
        }
    }
}

#[derive(Debug)]
struct App {
    ball: Ball,
}

impl App {
    fn new() -> Self {
        App { ball: Ball::new() }
    }
}

impl PistonApp for App {
    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.ball.step(state);
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            self.ball.draw(context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

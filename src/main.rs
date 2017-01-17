//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Vectors - Bouncing ball.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Ball {
    location: Vec2d,
    speed: Vec2d,
}

impl Ball {
    fn new() -> Self {
        Ball {
            location: [128.0, 128.0],
            speed: [2.0, 10.0 / 3.0],
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .color([0.5, 0.5, 0.5, 1.0])
            .draw(ellipse::circle(self.location[0], self.location[1], 32.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn step(&mut self, state: &PistonAppState) {
        self.location = math::add(self.location, self.speed);
        let (x, y) = (self.location[0], self.location[1]);
        if x > state.width() || x < 0.0 {
            self.speed[0] *= -1.0;
        }
        if y > state.height() || y < 0.0 {
            self.speed[1] *= -1.0;
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

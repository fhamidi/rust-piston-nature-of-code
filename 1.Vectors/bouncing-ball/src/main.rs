//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Vectors - Bouncing ball.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Ball {
    base_hue: Scalar,
    color_offset: Scalar,
    position: Vec2d,
    speed: Vec2d,
}

impl Ball {
    fn new() -> Self {
        let mut rng = thread_rng();
        Ball {
            base_hue: rng.gen(),
            color_offset: rng.gen(),
            position: [128.0, 128.0],
            speed: [2.0, 10.0 / 3.0],
        }
    }

    fn draw(&self, state: &PistonAppState, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .resolution(32)
            .color(state.noise_color(self.base_hue, self.color_offset, Some(1.0)))
            .draw(ellipse::circle(self.position[0], self.position[1], 32.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn update(&mut self, state: &PistonAppState) {
        self.color_offset += 1e-3;
        self.position = vec2_add(self.position, self.speed);
        let (x, y) = (self.position[0], self.position[1]);
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
        self.ball.update(state);
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            self.ball.draw(state, context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

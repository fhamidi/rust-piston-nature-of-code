//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Vectors - Bouncing ball.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Ball {
    color: Color,
    color_offset: Scalar,
    location: Vec2d,
    speed: Vec2d,
}

impl Ball {
    fn new() -> Self {
        Ball {
            color: color::TRANSPARENT,
            color_offset: SmallRng::from_entropy().gen(),
            location: [128.0, 128.0],
            speed: [2.0, 10.0 / 3.0],
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .resolution(32)
            .color(self.color)
            .draw(ellipse::circle(self.location[0], self.location[1], 32.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn update(&mut self, state: &PistonAppState) {
        self.color = state.noise_color(self.color_offset, Some(1.0));
        self.color_offset += 1e-3;
        self.location = vec2_add(self.location, self.speed);
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
        self.ball.update(state);
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

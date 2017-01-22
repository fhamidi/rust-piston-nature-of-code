//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Forces - Basic application of gravity and wind.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Mover {
    color: Color,
    location: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
    mass: Scalar,
}

impl Mover {
    fn new(x: Scalar, y: Scalar, mass: Scalar, color: Color) -> Self {
        Mover {
            color: color,
            location: [x, y],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
            mass: mass,
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .color(self.color)
            .draw(ellipse::circle(self.location[0], self.location[1], self.mass * 8.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn apply_force(&mut self, force: Vec2d) {
        self.acceleration = vec2_add(self.acceleration,
                                     vec2_scale(force, 1.0 / self.mass));
    }

    fn update(&mut self, state: &PistonAppState) {
        let (x, y) = (self.location[0], self.location[1]);
        let (width, height) = (state.width(), state.height());
        if x > width || x < 0.0 {
            self.location[0] = x.max(0.0).min(width);
            self.velocity[0] *= -1.0;
        }
        if y > height || y < 0.0 {
            self.location[1] = y.max(0.0).min(height);
            self.velocity[1] *= -1.0;
        }
        self.velocity = vec2_add(self.velocity, self.acceleration);
        self.location = vec2_add(self.location, self.velocity);
        self.acceleration = [0.0, 0.0];
    }
}

#[derive(Debug)]
struct App {
    movers: Vec<Mover>,
}

impl App {
    fn new() -> Self {
        App { movers: vec![] }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_MOVERS: usize = 32;
        let mut rng = rand::thread_rng();
        self.movers = (0..MAX_MOVERS)
            .map(|_| {
                Mover::new(0.0,
                           0.0,
                           rng.gen_range(0.1, 5.0),
                           state.random_color(Some(2.0 / 3.0)))
            })
            .collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        for mover in &mut self.movers {
            mover.apply_force([0.0, 0.1]);
            mover.apply_force([0.01, 0.0]);
            mover.update(state);
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            for mover in &self.movers {
                mover.draw(context, gfx);
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Forces - Helium ballon with wall bouncing and optional wind.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Mover {
    color: Color,
    location: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
}

impl Mover {
    fn new(state: &PistonAppState) -> Self {
        let mut rng = rand::thread_rng();
        Mover {
            color: state.random_color(Some(2.0 / 3.0)),
            location: [rng.gen_range(0.0, state.width()),
                       rng.gen_range(state.height() * 4.0 / 5.0, state.height())],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .color(self.color)
            .draw(ellipse::circle(self.location[0], self.location[1], 32.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn apply_force(&mut self, force: Vec2d) {
        self.acceleration = vec2_add(self.acceleration, force);
    }

    fn update(&mut self, state: &PistonAppState) {
        const BOUNCE_FACTOR: Scalar = -1.75;
        const REVERSE_GRAVITY: Scalar = -0.1;
        let (x, y) = (self.location[0], self.location[1]);
        let velocity = [self.velocity[0] + self.velocity[0].signum(),
                        self.velocity[1] + self.velocity[1].signum()];
        if x > state.width() || x < 0.0 {
            self.apply_force([velocity[0] * BOUNCE_FACTOR, 0.0]);
        }
        if y > state.height() || y < 0.0 {
            self.apply_force([0.0, velocity[1] * BOUNCE_FACTOR]);
        }
        self.apply_force([0.0, REVERSE_GRAVITY]);
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
        const MAX_MOVERS: usize = 1;
        self.movers = (0..MAX_MOVERS).map(|_| Mover::new(state)).collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        for mover in &mut self.movers {
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

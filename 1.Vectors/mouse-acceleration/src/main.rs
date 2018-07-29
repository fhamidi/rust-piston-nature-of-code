//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Vectors - Acceleration towards the mouse.

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
        let mut rng = SmallRng::from_entropy();
        Mover {
            color: state.random_color(Some(2.0 / 3.0)),
            location: [rng.gen_range(0.0, state.width()),
                       rng.gen_range(0.0, state.height())],
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

    fn update(&mut self, state: &PistonAppState) {
        const MAX_VELOCITY: Scalar = 4.2;
        let direction = vec2_sub([state.mouse_x(), state.mouse_y()], self.location);
        self.acceleration = vec2_scale(vec2_normalized(direction), 0.5);
        self.velocity = vec2_limit(vec2_add(self.velocity, self.acceleration),
                                   MAX_VELOCITY);
        self.location = vec2_add(self.location, self.velocity);
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
        const MAX_MOVERS: usize = 20;
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

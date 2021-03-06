//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Pointing in the direction of motion.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Mover {
    color: Color,
    position: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
}

impl Mover {
    fn new(state: &PistonAppState) -> Self {
        let mut rng = thread_rng();
        Mover {
            color: state.random_color(Some(2.0 / 3.0)),
            position: [
                rng.gen_range(0.0, state.width()),
                rng.gen_range(0.0, state.height()),
            ],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        let transform = context
            .transform
            .trans(self.position[0], self.position[1])
            .rot_rad(vec2_heading(self.velocity));
        Rectangle::new_border(color::BLACK, 1.0)
            .color(self.color)
            .draw(
                [-15.0, -5.0, 30.0, 10.0],
                &context.draw_state,
                transform,
                gfx,
            );
    }

    fn update(&mut self, state: &PistonAppState) {
        const MAX_VELOCITY: Scalar = 4.2;
        let direction = vec2_sub([state.mouse_x(), state.mouse_y()], self.position);
        self.acceleration = vec2_scale(vec2_normalized(direction), 0.5);
        self.velocity =
            vec2_limit(vec2_add(self.velocity, self.acceleration), MAX_VELOCITY);
        self.position = vec2_add(self.position, self.velocity);
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
        const MAX_MOVERS: usize = 16;
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

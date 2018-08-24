//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Simulation of a vehicle, driven by the arrow keys.

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
        Mover {
            color: state.random_color(Some(1.0)),
            position: [state.width() / 2.0, state.height() / 2.0],
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
            .draw([-15.0, -5.0, 30.0, 10.0],
                  &context.draw_state,
                  transform,
                  gfx);
    }

    fn update(&mut self, state: &PistonAppState) {
        const MAX_VELOCITY: Scalar = 4.2;
        if state.key_pressed() {
            match state.key() {
                Key::Left => self.acceleration[0] -= 0.01,
                Key::Right => self.acceleration[0] += 0.01,
                _ => (),
            }
        }
        self.velocity = vec2_limit(vec2_add(self.velocity, self.acceleration),
                                   MAX_VELOCITY);
        self.velocity[1] += 0.42;
        self.position = vec2_add(self.position, self.velocity);
        self.check_edges(state);
    }

    fn check_edges(&mut self, state: &PistonAppState) {
        let (x, y) = (self.position[0], self.position[1]);
        let (width, height) = (state.width(), state.height());
        if x > width {
            self.position[0] = 0.0;
        } else if x < 0.0 {
            self.position[0] = width;
        }
        if y > height {
            self.position[1] = 0.0;
        } else if y < 0.0 {
            self.position[1] = height;
        }
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
        self.movers.push(Mover::new(state));
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

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Oscillator objects.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Oscillator {
    color: Color,
    angle: Vec2d,
    velocity: Vec2d,
    amplitude: Vec2d,
}

impl Oscillator {
    fn new(state: &PistonAppState) -> Self {
        let mut rng = rand::thread_rng();
        Oscillator {
            color: state.random_color(Some(2.0 / 3.0)),
            angle: [0.0, 0.0],
            velocity: [rng.gen_range(-0.05, 0.05), rng.gen_range(-0.05, 0.05)],
            amplitude: [rng.gen_range(20.0, state.width() / 2.0),
                        rng.gen_range(20.0, state.height() / 2.0)],
        }
    }

    fn draw(&self, context: Context, transform: Matrix2d, gfx: &mut G2d) {
        let x = self.angle[0].sin() * self.amplitude[0];
        let y = self.angle[1].sin() * self.amplitude[1];
        Line::new(color::BLACK, 1.0)
            .draw([0.0, 0.0, x, y], &context.draw_state, transform, gfx);
        Ellipse::new_border(color::BLACK, 1.0)
            .color(self.color)
            .draw(ellipse::circle(x, y, 20.0),
                  &context.draw_state,
                  transform,
                  gfx);
    }

    fn update(&mut self) {
        self.angle = vec2_add(self.angle, self.velocity);
    }
}

#[derive(Debug)]
struct App {
    oscillators: Vec<Oscillator>,
}

impl App {
    fn new() -> Self {
        App { oscillators: vec![] }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_OSCILLATORS: usize = 16;
        self.oscillators = (0..MAX_OSCILLATORS)
            .map(|_| Oscillator::new(state))
            .collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        for oscillator in &mut self.oscillators {
            oscillator.update();
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            let transform = context.transform
                .trans(state.width() / 2.0, state.height() / 2.0);
            for oscillator in &self.oscillators {
                oscillator.draw(context, transform, gfx);
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

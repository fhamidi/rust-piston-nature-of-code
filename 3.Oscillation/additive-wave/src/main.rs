//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Additive wave.

extern crate piston_app;

use piston_app::*;

const SPACING: Scalar = 8.0;

#[derive(Debug)]
struct Wave {
    amplitude: Scalar,
    dx: Scalar,
}

impl Wave {
    fn new(amplitude: Scalar, period: Scalar) -> Self {
        Wave {
            amplitude: amplitude,
            dx: consts::PI * 2.0 / period * SPACING,
        }
    }
}

#[derive(Debug)]
struct App {
    theta: Scalar,
    waves: Vec<Wave>,
    y_values: Vec<Scalar>,
}

impl App {
    fn new() -> Self {
        const MAX_WAVES: usize = 6;
        let mut rng = rand::thread_rng();
        App {
            theta: 0.0,
            waves: (0..MAX_WAVES).map(|_| {
                Wave::new(rng.gen_range(10.0, 30.0), rng.gen_range(100.0, 300.0))
            }).collect(),
            y_values: vec![],
        }
    }
}

impl PistonApp for App {
    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.theta += 0.02;
        self.y_values = vec![0.0; (state.width() / SPACING + 16.0) as usize];
        for (i, wave) in self.waves.iter().enumerate() {
            let mut theta = self.theta;
            for j in 0..self.y_values.len() {
                self.y_values[j] += if i % 2 == 0 {
                    theta.sin() * wave.amplitude
                } else {
                    theta.cos() * wave.amplitude
                };
                theta += wave.dx;
            }
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            for (x, y) in self.y_values.iter().enumerate() {
                Ellipse::new_border(color::BLACK, 1.0)
                    .color([0.0, 0.0, 0.0, 1.0 / 3.0])
                    .draw(ellipse::circle(x as Scalar * SPACING,
                                          state.height() / 2.0 + y,
                                          24.0),
                          &context.draw_state,
                          context.transform,
                          gfx);
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

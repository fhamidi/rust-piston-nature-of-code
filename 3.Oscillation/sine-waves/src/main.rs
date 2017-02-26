//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Sine waves.

extern crate piston_app;

use piston_app::*;

const SPACING: Scalar = 8.0;

#[derive(Debug)]
struct Wave {
    origin: Vec2d,
    width: Scalar,
    theta: Scalar,
    amplitude: Scalar,
    period: Scalar,
    dx: Scalar,
    y_values: Vec<Scalar>,
}

impl Wave {
    fn new(origin: Vec2d, width: Scalar, amplitude: Scalar, period: Scalar) -> Self {
        Wave {
            origin: origin,
            width: width,
            theta: 0.0,
            amplitude: amplitude,
            period: period,
            dx: consts::PI * 2.0 / period * SPACING,
            y_values: vec![0.0; (width / SPACING) as usize],
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        for (x, y) in self.y_values.iter().enumerate() {
            Ellipse::new_border(color::BLACK, 1.0)
                .color([0.0, 0.0, 0.0, 1.0 / 3.0])
                .draw(ellipse::circle(self.origin[0] + x as Scalar * SPACING,
                                      self.origin[1] + y,
                                      24.0),
                      &context.draw_state,
                      context.transform,
                      gfx);
        }
    }

    fn update(&mut self) {
        self.theta += 0.042;
        let mut x = self.theta;
        for y in &mut self.y_values {
            *y = x.sin() * self.amplitude;
            x += self.dx;
        }
    }
}

#[derive(Debug)]
struct App {
    waves: Vec<Wave>,
}

impl App {
    fn new() -> Self {
        App {
            waves: vec![Wave::new([50.0, 180.0], 100.0, 20.0, 500.0),
                        Wave::new([300.0, 240.0], 300.0, 40.0, 220.0)],
        }
    }
}

impl PistonApp for App {
    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        for wave in &mut self.waves {
            wave.update();
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            for wave in &self.waves {
                wave.draw(context, gfx);
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}
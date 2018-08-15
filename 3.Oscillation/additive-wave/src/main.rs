//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Additive wave.

extern crate piston_app;

use piston_app::*;

const SPACING: Scalar = 4.2;

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
    node_texture: Option<G2dTexture>,
    theta: Scalar,
    waves: Vec<Wave>,
    y_values: Vec<Scalar>,
}

impl App {
    fn new() -> Self {
        const MAX_WAVES: usize = 6;
        let mut rng = SmallRng::from_entropy();
        App {
            node_texture: None,
            theta: 0.0,
            waves: (0..MAX_WAVES)
                .map(|_| {
                         Wave::new(rng.gen_range(12.0, 42.0), rng.gen_range(120.0, 240.0))
                     })
                .collect(),
            y_values: vec![],
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, _: &PistonAppState) {
        self.node_texture = Some(Texture::from_path(&mut window.factory,
                                                    "assets/node.png",
                                                    Flip::None,
                                                    &TextureSettings::new())
                                     .unwrap());
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.theta += 0.02;
        self.y_values = vec![0.0; (state.width() / SPACING) as usize];
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
            let node_texture = self.node_texture.as_ref().unwrap();
            for (x, y) in self.y_values.iter().enumerate() {
                state.draw_centered_texture(node_texture,
                                            x as Scalar * SPACING,
                                            state.height() / 2.0 + y,
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

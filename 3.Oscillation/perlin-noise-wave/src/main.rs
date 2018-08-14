//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Perlin noise wave.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct App {
    node_texture: Option<G2dTexture>,
    start_angle: Scalar,
    velocity: Scalar,
}

impl App {
    fn new() -> Self {
        App {
            node_texture: None,
            start_angle: 0.0,
            velocity: 0.05,
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
        window.draw_2d(state.event(), |context, gfx| {
            const NODE_RADIUS: Scalar = 24.0;
            clear(color::WHITE, gfx);
            let node_texture = self.node_texture.as_ref().unwrap();
            let mut angle = self.start_angle;
            self.start_angle += 0.02;
            let mut x = 0.0;
            while x <= state.width() {
                let y =
                    state.map_range(state.noise(&[angle]), 0.0, 1.0, 0.0, state.height());
                let transform = context.transform.trans(x - NODE_RADIUS, y - NODE_RADIUS);
                image(node_texture, transform, gfx);
                angle += self.velocity;
                x += 8.0;
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Perlin noise wave.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct App {
    node_texture: Option<G2dTexture>,
    base_hue: Scalar,
    color_offset: Scalar,
    start_angle: Scalar,
    velocity: Scalar,
}

impl App {
    fn new() -> Self {
        let mut rng = thread_rng();
        App {
            base_hue: rng.gen(),
            node_texture: None,
            color_offset: rng.gen(),
            start_angle: 0.0,
            velocity: 0.05,
        }
    }

    fn node_texture(&self) -> &G2dTexture {
        self.node_texture.as_ref().unwrap()
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, _: &PistonAppState) {
        self.node_texture = Some(
            Texture::from_path(
                &mut window.factory,
                "assets/node.png",
                Flip::None,
                &TextureSettings::new(),
            ).unwrap(),
        );
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        let mut color_offset = self.color_offset;
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            let mut angle = self.start_angle;
            self.start_angle += 0.02;
            let node_texture = self.node_texture();
            let mut x = 0.0;
            while x <= state.width() {
                color_offset += 1e-3;
                let y =
                    state.map_range(state.noise(&[angle]), 0.0, 1.0, 0.0, state.height());
                state.draw_centered_texture(
                    node_texture,
                    Some(state.noise_color(self.base_hue, color_offset, Some(1.0))),
                    x,
                    y,
                    &context.draw_state,
                    context.transform,
                    gfx,
                );
                angle += self.velocity;
                x += 8.0;
            }
        });
        self.color_offset += 1e-3;
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

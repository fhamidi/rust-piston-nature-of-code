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
        let mut rng = thread_rng();
        let uniform = Uniform::new_inclusive(-0.05, 0.05);
        Oscillator {
            color: state.random_color(Some(1.0)),
            angle: [0.0, 0.0],
            velocity: [rng.sample(uniform), rng.sample(uniform)],
            amplitude: [
                rng.gen_range(20.0, state.width() / 2.0),
                rng.gen_range(20.0, state.height() / 2.0),
            ],
        }
    }

    fn draw(
        &self,
        node_texture: &G2dTexture,
        state: &PistonAppState,
        context: Context,
        transform: Matrix2d,
        gfx: &mut G2d,
    ) {
        let x = self.angle[0].sin() * self.amplitude[0];
        let y = self.angle[1].sin() * self.amplitude[1];
        Line::new(color::BLACK, 1.0).draw(
            [0.0, 0.0, x, y],
            &context.draw_state,
            transform,
            gfx,
        );
        state.draw_centered_texture(
            node_texture,
            Some(self.color),
            x,
            y,
            &context.draw_state,
            transform,
            gfx,
        );
    }

    fn update(&mut self) {
        self.angle = vec2_add(self.angle, self.velocity);
    }
}

#[derive(Debug)]
struct App {
    node_texture: Option<G2dTexture>,
    oscillators: Vec<Oscillator>,
}

impl App {
    fn new() -> Self {
        App {
            node_texture: None,
            oscillators: vec![],
        }
    }

    fn node_texture(&self) -> &G2dTexture {
        self.node_texture.as_ref().unwrap()
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_OSCILLATORS: usize = 32;
        self.node_texture = Some(
            Texture::from_path(
                &mut window.factory,
                "assets/node.png",
                Flip::None,
                &TextureSettings::new(),
            ).unwrap(),
        );
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
            let node_texture = self.node_texture();
            let transform = context
                .transform
                .trans(state.width() / 2.0, state.height() / 2.0);
            for oscillator in &self.oscillators {
                oscillator.draw(node_texture, state, context, transform, gfx);
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

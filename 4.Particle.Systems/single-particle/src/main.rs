//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Particle systems - Single particle.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Particle {
    color: Color,
    position: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
    life: Scalar,
}

impl Particle {
    fn new(color: Color, x: Scalar, y: Scalar) -> Self {
        let mut rng = SmallRng::from_entropy();
        Particle {
            color: color,
            position: [x, y],
            velocity: [rng.gen_range(-1.0, 1.0), rng.gen_range(-2.0, 0.0)],
            acceleration: [0.0, 0.05],
            life: 1.0,
        }
    }

    #[inline]
    fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    fn draw(&self,
            texture: &G2dTexture,
            state: &PistonAppState,
            context: Context,
            gfx: &mut G2d) {
        state.draw_centered_texture(texture,
                                    Some([self.color[0],
                                          self.color[1],
                                          self.color[2],
                                          self.life as ColorComponent]),
                                    self.position[0],
                                    self.position[1],
                                    &context.draw_state,
                                    context.transform,
                                    gfx);
    }

    fn update(&mut self) {
        self.velocity = vec2_add(self.velocity, self.acceleration);
        self.position = vec2_add(self.position, self.velocity);
        self.life -= 1.0 / 128.0;
    }
}

#[derive(Debug)]
struct App {
    particle_texture: Option<G2dTexture>,
    particle: Option<Particle>,
}

impl App {
    fn new() -> Self {
        App {
            particle_texture: None,
            particle: None,
        }
    }

    fn particle_texture(&self) -> &G2dTexture {
        self.particle_texture.as_ref().unwrap()
    }

    fn particle(&self) -> &Particle {
        self.particle.as_ref().unwrap()
    }

    fn particle_mut(&mut self) -> &mut Particle {
        self.particle.as_mut().unwrap()
    }

    fn spawn_particle(&mut self, state: &PistonAppState) {
        self.particle =
            Some(Particle::new(state.random_color(Some(1.0)), state.width() / 2.0, 42.0));
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.particle_texture = Some(Texture::from_path(&mut window.factory,
                                                        "assets/particle.png",
                                                        Flip::None,
                                                        &TextureSettings::new())
                                         .unwrap());
        self.spawn_particle(state);
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.particle_mut().update();
        if !self.particle().is_alive() {
            self.spawn_particle(state);
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            self.particle()
                .draw(self.particle_texture(), state, context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

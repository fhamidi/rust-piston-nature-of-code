//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Particle systems - Multiple particle systems.

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
    fn new(color: Color, position: Vec2d) -> Self {
        let mut rng = SmallRng::from_entropy();
        Particle {
            color: color,
            position: position,
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
struct ParticleSystem {
    color_offset: Scalar,
    origin: Vec2d,
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn new(x: Scalar, y: Scalar) -> Self {
        ParticleSystem {
            color_offset: SmallRng::from_entropy().gen(),
            origin: [x, y],
            particles: vec![],
        }
    }

    fn draw(&self,
            texture: &G2dTexture,
            state: &PistonAppState,
            context: Context,
            gfx: &mut G2d) {
        for particle in &self.particles {
            particle.draw(texture, state, context, gfx);
        }
    }

    fn spawn_particle(&mut self, state: &PistonAppState) {
        self.color_offset += 0.00042;
        self.particles
            .push(Particle::new(state.noise_color(self.color_offset, Some(1.0)),
                                self.origin));
    }

    fn update(&mut self, state: &PistonAppState) {
        for particle in &mut self.particles {
            particle.update();
        }
        self.particles.retain(|ref particle| particle.is_alive());
        self.spawn_particle(state);
    }
}

#[derive(Debug)]
struct App {
    particle_texture: Option<G2dTexture>,
    particle_systems: Vec<ParticleSystem>,
}

impl App {
    fn new() -> Self {
        App {
            particle_texture: None,
            particle_systems: vec![],
        }
    }

    fn particle_texture(&self) -> &G2dTexture {
        self.particle_texture.as_ref().unwrap()
    }

    fn spawn_particle_system(&mut self, state: &PistonAppState) {
        self.particle_systems
            .push(ParticleSystem::new(state.mouse_x(), state.mouse_y()));
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_INITIAL_SYSTEMS: usize = 3;
        self.particle_texture = Some(Texture::from_path(&mut window.factory,
                                                        "assets/particle.png",
                                                        Flip::None,
                                                        &TextureSettings::new())
                                         .unwrap());
        let mut rng = SmallRng::from_entropy();
        self.particle_systems = (0..MAX_INITIAL_SYSTEMS)
            .map(|_| {
                     ParticleSystem::new(rng.gen_range(42.0, state.width() - 42.0),
                                         rng.gen_range(42.0, state.height() - 42.0))
                 })
            .collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        if state.mouse_button_clicked(MouseButton::Left) {
            self.spawn_particle_system(state);
        }
        for particle_system in &mut self.particle_systems {
            particle_system.update(state);
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            for particle_system in &self.particle_systems {
                particle_system.draw(self.particle_texture(), state, context, gfx);
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

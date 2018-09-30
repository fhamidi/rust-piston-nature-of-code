//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Particle systems - Particle system with forces.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Particle {
    color: Color,
    position: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
    mass: Scalar,
    life: Scalar,
}

impl Particle {
    fn new(color: Color, position: Vec2d) -> Self {
        let mut rng = thread_rng();
        Particle {
            color: color,
            position: position,
            velocity: [rng.gen_range(-1.0, 1.0), rng.gen_range(-2.0, 0.0)],
            acceleration: [0.0, 0.0],
            mass: 1.0,
            life: 1.0,
        }
    }

    #[inline]
    fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    fn draw(
        &self,
        texture: &G2dTexture,
        state: &PistonAppState,
        context: Context,
        gfx: &mut G2d,
    ) {
        state.draw_centered_texture(
            texture,
            Some([
                self.color[0],
                self.color[1],
                self.color[2],
                self.life as ColorComponent,
            ]),
            self.position[0],
            self.position[1],
            &context.draw_state,
            context.transform,
            gfx,
        );
    }

    fn apply_force(&mut self, force: Vec2d) {
        self.acceleration =
            vec2_add(self.acceleration, vec2_scale(force, 1.0 / self.mass));
    }

    fn update(&mut self) {
        self.velocity = vec2_add(self.velocity, self.acceleration);
        self.position = vec2_add(self.position, self.velocity);
        self.acceleration = [0.0, 0.0];
        self.life -= 1.0 / 128.0;
    }
}

#[derive(Debug)]
struct ParticleSystem {
    base_hue: Scalar,
    color_offset: Scalar,
    origin: Vec2d,
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn new(x: Scalar, y: Scalar) -> Self {
        let mut rng = thread_rng();
        ParticleSystem {
            base_hue: rng.gen(),
            color_offset: rng.gen(),
            origin: [x, y],
            particles: vec![],
        }
    }

    fn draw(
        &self,
        texture: &G2dTexture,
        state: &PistonAppState,
        context: Context,
        gfx: &mut G2d,
    ) {
        for particle in &self.particles {
            particle.draw(texture, state, context, gfx);
        }
    }

    fn apply_force(&mut self, force: Vec2d) {
        for particle in &mut self.particles {
            particle.apply_force(force);
        }
    }

    fn spawn_particle(&mut self, state: &PistonAppState) {
        self.color_offset += 0.00042;
        self.particles.push(Particle::new(
            state.noise_color(self.base_hue, self.color_offset, Some(1.0)),
            self.origin,
        ));
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
    particle_system: Option<ParticleSystem>,
}

impl App {
    fn new() -> Self {
        App {
            particle_texture: None,
            particle_system: None,
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.particle_texture = Some(
            Texture::from_path(
                &mut window.factory,
                "assets/particle.png",
                Flip::None,
                &TextureSettings::new(),
            ).unwrap(),
        );
        self.particle_system = Some(ParticleSystem::new(state.width() / 2.0, 42.0));
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        const GRAVITY: Vec2d = [0.0, 0.1];
        let particle_texture = self.particle_texture.as_ref().unwrap();
        let particle_system = self.particle_system.as_mut().unwrap();
        particle_system.apply_force(GRAVITY);
        particle_system.update(state);
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            particle_system.draw(particle_texture, state, context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

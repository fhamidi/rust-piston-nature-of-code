//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Particle systems - Single particle system.

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
        let mut rng = thread_rng();
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

    fn update(&mut self) {
        self.velocity = vec2_add(self.velocity, self.acceleration);
        self.position = vec2_add(self.position, self.velocity);
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

    fn particle_texture(&self) -> &G2dTexture {
        self.particle_texture.as_ref().unwrap()
    }

    fn particle_system(&self) -> &ParticleSystem {
        self.particle_system.as_ref().unwrap()
    }

    fn particle_system_mut(&mut self) -> &mut ParticleSystem {
        self.particle_system.as_mut().unwrap()
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
            )
            .unwrap(),
        );
        self.particle_system = Some(ParticleSystem::new(state.width() / 2.0, 42.0));
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.particle_system_mut().update(state);
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            self.particle_system()
                .draw(self.particle_texture(), state, context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

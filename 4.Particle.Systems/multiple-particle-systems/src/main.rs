//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Particle systems - Multiple particle systems.

#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate piston_app;

use gfx::preset;
use gfx::state::ColorMask;
use gfx::traits::FactoryExt;
use piston_app::*;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "pos",
        uv: [f32; 2] = "uv",
        color: [f32; 4] = "color",
    }

    pipeline particles {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "tex",
        out: gfx::BlendTarget<gfx::format::Srgba8> = ("o_Color",
                                                      ColorMask::all(),
                                                      preset::blend::ALPHA),
    }
}

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

    #[inline]
    fn normalize_x(&self, state: &PistonAppState, x: Scalar) -> f32 {
        state.map_range(x, 0.0, state.width(), -1.0, 1.0) as f32
    }

    #[inline]
    fn normalize_y(&self, state: &PistonAppState, y: Scalar) -> f32 {
        -state.map_range(y, 0.0, state.height(), -1.0, 1.0) as f32
    }

    fn extend_vertex_buffer(&self,
                            state: &PistonAppState,
                            vertices: &mut Vec<Vertex>,
                            indices: &mut Vec<u32>) {
        const HALF_SIDE: Scalar = 8.0;
        let start = vertices.len() as u32;
        let color = [self.color[0],
                     self.color[1],
                     self.color[2],
                     self.life as ColorComponent];
        let (x, y) = (self.position[0], self.position[1]);
        vertices.extend(&[Vertex {
                              pos: [self.normalize_x(state, x + HALF_SIDE),
                                    self.normalize_y(state, y + HALF_SIDE)],
                              uv: [1.0, 1.0],
                              color: color,
                          },
                          Vertex {
                              pos: [self.normalize_x(state, x - HALF_SIDE),
                                    self.normalize_y(state, y + HALF_SIDE)],
                              uv: [0.0, 1.0],
                              color: color,
                          },
                          Vertex {
                              pos: [self.normalize_x(state, x - HALF_SIDE),
                                    self.normalize_y(state, y - HALF_SIDE)],
                              uv: [0.0, 0.0],
                              color: color,
                          },
                          Vertex {
                              pos: [self.normalize_x(state, x + HALF_SIDE),
                                    self.normalize_y(state, y - HALF_SIDE)],
                              uv: [1.0, 0.0],
                              color: color,
                          }]);
        indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start]);
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

    #[inline]
    fn len(&self) -> usize {
        self.particles.len()
    }

    fn extend_vertex_buffer(&self,
                            state: &PistonAppState,
                            vertices: &mut Vec<Vertex>,
                            indices: &mut Vec<u32>) {
        for particle in &self.particles {
            particle.extend_vertex_buffer(state, vertices, indices);
        }
    }

    fn spawn_particle(&mut self, state: &PistonAppState) {
        self.color_offset += 0.00042;
        self.particles
            .push(Particle::new(state.noise_color(self.base_hue,
                                                  self.color_offset,
                                                  Some(1.0)),
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
    pipeline: Option<gfx::pso::PipelineState<gfx_device_gl::Resources, particles::Meta>>,
    particle_systems: Vec<ParticleSystem>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl App {
    fn new() -> Self {
        App {
            particle_texture: None,
            pipeline: None,
            particle_systems: vec![],
            vertices: Vec::with_capacity(4 * 4096),
            indices: Vec::with_capacity(6 * 4096),
        }
    }

    fn particle_texture(&self) -> &G2dTexture {
        self.particle_texture.as_ref().unwrap()
    }

    fn pipeline
        (&self)
         -> &gfx::pso::PipelineState<gfx_device_gl::Resources, particles::Meta> {
        self.pipeline.as_ref().unwrap()
    }

    fn spawn_particle_system(&mut self, state: &PistonAppState) {
        self.particle_systems
            .push(ParticleSystem::new(state.mouse_x(), state.mouse_y()));
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_INITIAL_PARTICLE_SYSTEMS: usize = 9;
        let factory = &mut window.factory;
        self.particle_texture = Some(Texture::from_path(factory,
                                                        "assets/particle.png",
                                                        Flip::None,
                                                        &TextureSettings::new())
                                         .unwrap());
        self.pipeline =
            Some(factory
                     .create_pipeline_simple(include_bytes!("particle_150_core.glslv"),
                                             include_bytes!("particle_150_core.glslf"),
                                             particles::new())
                     .unwrap());
        let mut rng = thread_rng();
        self.particle_systems = (0..MAX_INITIAL_PARTICLE_SYSTEMS)
            .map(|_| {
                     ParticleSystem::new(rng.gen_range(42.0, state.width() - 42.0),
                                         rng.gen_range(42.0, state.height() - 42.0))
                 })
            .collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        if state.key_hit(Key::D) {
            let total_particle_count: usize = self.particle_systems
                .iter()
                .map(|particle_system| particle_system.len())
                .sum();
            println!("Frame {} | Particle systems: {} | Total particles: {}",
                     state.frame_count(),
                     self.particle_systems.len(),
                     total_particle_count);
        }
        if state.mouse_button_clicked(MouseButton::Left) {
            self.spawn_particle_system(state);
        }
        self.vertices.clear();
        self.indices.clear();
        for particle_system in &mut self.particle_systems {
            particle_system.update(state);
            particle_system
                .extend_vertex_buffer(state, &mut self.vertices, &mut self.indices);
        }
        let (vbuf, slice) =
            window
                .factory
                .create_vertex_buffer_with_slice(&self.vertices[..], &self.indices[..]);
        let texture = self.particle_texture();
        let encoder = &mut window.encoder;
        encoder.clear(&window.output_color, color::WHITE);
        encoder.draw(&slice,
                     self.pipeline(),
                     &particles::Data {
                         vbuf: vbuf,
                         texture: (texture.view.clone(), texture.sampler.clone()),
                         out: window.output_color.clone(),
                     });
        encoder.flush(&mut window.device);
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

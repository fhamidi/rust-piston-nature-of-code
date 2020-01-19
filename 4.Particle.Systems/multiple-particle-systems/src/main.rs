//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Particle systems - Multiple particle systems.

extern crate piston_app;

use piston_app::*;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "pos",
        uv: [f32; 2] = "uv",
        color: [f32; 4] = "color",
    }

    pipeline particles {
        vbuf: VertexBuffer<Vertex> = (),
        sampler: TextureSampler<[f32; 4]> = "sampler",
        out: BlendTarget<gfx::format::Srgba8> = ("o_color",
                                                 gfx::state::ColorMask::all(),
                                                 gfx::preset::blend::ALPHA),
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

    fn extend_vertex_buffer(
        &self,
        state: &PistonAppState,
        texture_atlas: &TextureAtlas,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
    ) {
        let start = vertices.len() as u32;
        let color = [
            self.color[0],
            self.color[1],
            self.color[2],
            self.life as ColorComponent,
        ];
        let (x, y) = (self.position[0], self.position[1]);
        let (w, h) = texture_atlas.texture_offsets(0);
        vertices.extend(&[
            Vertex {
                pos: [
                    state.normalize_x(x + w) as f32,
                    state.normalize_y(y + h) as f32,
                ],
                uv: [1.0, 1.0],
                color: color,
            },
            Vertex {
                pos: [
                    state.normalize_x(x - w) as f32,
                    state.normalize_y(y + h) as f32,
                ],
                uv: [0.0, 1.0],
                color: color,
            },
            Vertex {
                pos: [
                    state.normalize_x(x - w) as f32,
                    state.normalize_y(y - h) as f32,
                ],
                uv: [0.0, 0.0],
                color: color,
            },
            Vertex {
                pos: [
                    state.normalize_x(x + w) as f32,
                    state.normalize_y(y - h) as f32,
                ],
                uv: [1.0, 0.0],
                color: color,
            },
        ]);
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

    fn extend_vertex_buffer(
        &self,
        state: &PistonAppState,
        texture_atlas: &TextureAtlas,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
    ) {
        for particle in &self.particles {
            particle.extend_vertex_buffer(state, texture_atlas, vertices, indices);
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
    particle_systems: Vec<ParticleSystem>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    pipeline: Option<PistonPipeline<particles::Meta>>,
    renderer: Option<PistonRenderer>,
}

impl App {
    fn new() -> Self {
        App {
            particle_systems: vec![],
            vertices: Vec::with_capacity(4 * 4096),
            indices: Vec::with_capacity(6 * 4096),
            pipeline: None,
            renderer: None,
        }
    }

    fn pipeline(&self) -> &PistonPipeline<particles::Meta> {
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
        let mut rng = thread_rng();
        self.particle_systems = (0..MAX_INITIAL_PARTICLE_SYSTEMS)
            .map(|_| {
                ParticleSystem::new(
                    rng.gen_range(42.0, state.width() - 42.0),
                    rng.gen_range(42.0, state.height() - 42.0),
                )
            })
            .collect();
        let (pipeline, renderer) = PistonPipelineBuilder::new()
            .texture_atlas(
                TextureAtlas::from_path(window, "assets/particle.png").unwrap(),
            )
            .vertex_shader(include_bytes!("particles_150_core.glslv"))
            .fragment_shader(include_bytes!("particles_150_core.glslf"))
            .build(window, particles::new())
            .unwrap();
        self.pipeline = Some(pipeline);
        self.renderer = Some(renderer);
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        if state.key_hit(Key::D) {
            let total_particle_count: usize = self
                .particle_systems
                .iter()
                .map(|particle_system| particle_system.len())
                .sum();
            println!(
                "Frame {} | Particle systems: {} | Total particles: {}",
                state.frame_count(),
                self.particle_systems.len(),
                total_particle_count
            );
        }
        if state.mouse_button_clicked(MouseButton::Left) {
            self.spawn_particle_system(state);
        }
        self.vertices.clear();
        self.indices.clear();
        let renderer = self.renderer.as_ref().unwrap();
        let texture_atlas = renderer.texture_atlas().unwrap();
        for particle_system in &mut self.particle_systems {
            particle_system.update(state);
            particle_system.extend_vertex_buffer(
                state,
                texture_atlas,
                &mut self.vertices,
                &mut self.indices,
            );
        }
        renderer.clear(window, color::WHITE);
        renderer.draw(
            window,
            self.pipeline(),
            &self.vertices[..],
            &self.indices[..],
            |vbuf, out| particles::Data {
                vbuf: vbuf,
                sampler: texture_atlas.texture_view_sampler(),
                out: out,
            },
        );
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

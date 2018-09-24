//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Physics libraries - Fixed boundaries.

extern crate piston_app;
extern crate wrapped2d;

use piston_app::*;
use wrapped2d::b2;

const PIXELS_PER_METER: f32 = 32.0;
type World = b2::World<wrapped2d::user_data::NoUserData>;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "pos",
        uv: [f32; 2] = "uv",
        color: [f32; 4] = "color",
    }

    pipeline world {
        vbuf: VertexBuffer<Vertex> = (),
        sampler: TextureSampler<[f32; 4]> = "sampler",
        transform: Global<[f32; 4]> = "transform",
        out: RenderTarget<gfx::format::Srgba8> = "o_Color",
    }
}

#[derive(Debug)]
struct Boundary {
    body_handle: b2::BodyHandle,
    x: f32,
    y: f32,
    half_width: f32,
    half_height: f32,
}

impl Boundary {
    fn new(world: &mut World, x: f32, y: f32, width: f32, height: f32) -> Self {
        let handle = world.create_body(&b2::BodyDef {
                                           position: b2::Vec2 { x: x, y: y },
                                           ..b2::BodyDef::new()
                                       });
        let mut body = world.body_mut(handle);
        let (half_width, half_height) = (width / 2.0, height / 2.0);
        body.create_fast_fixture(&b2::PolygonShape::new_box(half_width, half_height),
                                 0.0);
        Boundary {
            body_handle: handle,
            x: x,
            y: y,
            half_width: half_width,
            half_height: half_height,
        }
    }

    fn extend_vertex_buffer(&self, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
        let start = vertices.len() as u32;
        let (x, y) = (self.x, self.y);
        let (w, h) = (self.half_width + 0.02, self.half_height + 0.02);
        vertices.extend(&[Vertex {
                              pos: [x + w, y + h],
                              uv: [0.0, 0.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [x - w, y + h],
                              uv: [0.0, 0.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [x - w, y - h],
                              uv: [0.0, 0.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [x + w, y - h],
                              uv: [0.0, 0.0],
                              color: color::BLACK,
                          }]);
        indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start]);
    }
}

#[derive(Debug)]
struct Brick {
    body_handle: b2::BodyHandle,
    half_width: f32,
    half_height: f32,
    color: Color,
}

impl Brick {
    fn new(world: &mut World,
           x: f32,
           y: f32,
           width: f32,
           height: f32,
           color: Color)
           -> Self {
        let handle = world.create_body(&b2::BodyDef {
                                           body_type: b2::BodyType::Dynamic,
                                           position: b2::Vec2 { x: x, y: y },
                                           ..b2::BodyDef::new()
                                       });
        let mut body = world.body_mut(handle);
        let (half_width, half_height) = (width / 2.0, height / 2.0);
        body.create_fixture(&b2::PolygonShape::new_box(half_width, half_height),
                            &mut b2::FixtureDef {
                                density: 1.0,
                                friction: 0.3,
                                restitution: 0.5,
                                ..b2::FixtureDef::new()
                            });
        Brick {
            body_handle: handle,
            half_width: half_width,
            half_height: half_height,
            color: color,
        }
    }

    fn extend_vertex_buffer(&self,
                            world: &World,
                            vertices: &mut Vec<Vertex>,
                            indices: &mut Vec<u32>) {
        let start = vertices.len() as u32;
        let body = world.body(self.body_handle);
        let transform = body.transform();
        let (w, h) = (self.half_width + 0.02, self.half_height + 0.02);
        let (iw, ih) = (w - 0.075, h - 0.075);
        vertices.extend(&[Vertex {
                              pos: *(transform * b2::Vec2 { x: w, y: h }).as_array(),
                              uv: [0.0, 0.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: -w, y: h }).as_array(),
                              uv: [0.0, 0.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: -w, y: -h }).as_array(),
                              uv: [0.0, 0.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: w, y: -h }).as_array(),
                              uv: [0.0, 0.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: iw, y: ih }).as_array(),
                              uv: [0.5, 0.5],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: -iw, y: ih }).as_array(),
                              uv: [0.5, 0.5],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: -iw, y: -ih }).as_array(),
                              uv: [0.5, 0.5],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: iw, y: -ih }).as_array(),
                              uv: [0.5, 0.5],
                              color: self.color,
                          }]);
        indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start,
                         start + 4, start + 5, start + 6, start + 6, start + 7,
                         start + 4]);
    }
}

struct App {
    world: Option<World>,
    boundaries: Vec<Boundary>,
    bricks: Vec<Brick>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    pipeline: Option<PistonPipeline<world::Meta>>,
    renderer: Option<PistonRenderer>,
}

impl App {
    fn new() -> Self {
        App {
            world: None,
            boundaries: vec![],
            bricks: vec![],
            vertices: Vec::with_capacity(4 * 4096),
            indices: Vec::with_capacity(6 * 4096),
            pipeline: None,
            renderer: None,
        }
    }

    fn dump_data(&self, state: &PistonAppState) {
        let boundary_count = self.boundaries.len();
        let brick_count = self.bricks.len();
        let vertex_count = self.vertices.len();
        let index_count = self.indices.len();
        let memory = (boundary_count * std::mem::size_of::<Boundary>() +
                      brick_count * std::mem::size_of::<Brick>() +
                      vertex_count * std::mem::size_of::<Vertex>() +
                      index_count * std::mem::size_of::<u32>()) as
                     f32 / 1024.0;
        println!("Frame {} | Bricks: {} | Vertices: {} | Indices: {} | Memory: {:.2} KB",
                 state.frame_count(),
                 brick_count,
                 vertex_count,
                 index_count,
                 memory);
    }

    fn setup_world(&mut self, state: &PistonAppState) {
        let gravity = b2::Vec2 { x: 0.0, y: -10.0 };
        let mut world = World::new(&gravity);
        let ground = world.create_body(&b2::BodyDef {
                                           position: b2::Vec2 { x: 0.0, y: -10.0 },
                                           ..b2::BodyDef::new()
                                       });
        let width = state.width() as f32;
        let shape = b2::PolygonShape::new_box(width * 4.2 / PIXELS_PER_METER, 10.0);
        world.body_mut(ground).create_fast_fixture(&shape, 0.0);
        let boundary_width = width / 2.0 / PIXELS_PER_METER - 2.0;
        self.boundaries = vec![Boundary::new(&mut world,
                                             -boundary_width / 2.0 - 1.0,
                                             2.0,
                                             boundary_width,
                                             0.5),
                               Boundary::new(&mut world,
                                             boundary_width / 2.0 + 1.0,
                                             4.0,
                                             boundary_width,
                                             0.5)];
        self.world = Some(world);
    }

    fn spawn_brick(&mut self, state: &PistonAppState) {
        let mut rng = thread_rng();
        let uniform = Uniform::new_inclusive(0.2, 1.0);
        let x = (state.mouse_x() - state.width() / 2.0) as f32 / PIXELS_PER_METER;
        let y = (state.height() - state.mouse_y()) as f32 / PIXELS_PER_METER;
        let brick = Brick::new(self.world.as_mut().unwrap(),
                               x,
                               y,
                               rng.sample(uniform),
                               rng.sample(uniform),
                               state.random_color(Some(1.0)));
        self.bricks.push(brick);
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.setup_world(state);
        let (pipeline, renderer) = PistonPipelineBuilder::new()
            .texture_atlas(TextureAtlas::from_path(window, "assets/brick.png").unwrap())
            .vertex_shader(include_bytes!("world_150_core.glslv"))
            .fragment_shader(include_bytes!("world_150_core.glslf"))
            .build(window, world::new())
            .unwrap();
        self.pipeline = Some(pipeline);
        self.renderer = Some(renderer);
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        if state.key_hit(Key::D) {
            self.dump_data(state);
        }
        if state.mouse_button_pressed(MouseButton::Left) {
            self.spawn_brick(state);
        }
        self.vertices.clear();
        self.indices.clear();
        let world = self.world.as_mut().unwrap();
        world.step(1.0 / 60.0, 8, 3);
        world.clear_forces();
        for boundary in &self.boundaries {
            boundary.extend_vertex_buffer(&mut self.vertices, &mut self.indices);
        }
        for brick in &self.bricks {
            brick.extend_vertex_buffer(world, &mut self.vertices, &mut self.indices);
        }
        let renderer = self.renderer.as_ref().unwrap();
        let texture_atlas = renderer.texture_atlas().unwrap();
        let half_width = state.width() as f32 / 2.0;
        let half_height = state.height() as f32 / 2.0;
        renderer.clear(window, color::WHITE);
        renderer.draw(
            window,
            self.pipeline.as_ref().unwrap(),
            &self.vertices[..],
            &self.indices[..],
            |vbuf, out| {
                world::Data {
                    vbuf: vbuf,
                    sampler: texture_atlas.texture_view_sampler(),
                    transform: [0.0,
                                -1.0,
                                PIXELS_PER_METER / half_width,
                                PIXELS_PER_METER / half_height],
                    out: out,
                }
            },
        );
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

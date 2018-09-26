//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Physics libraries - Sloped boundary.

extern crate piston_app;
extern crate wrapped2d;

use piston_app::*;
use wrapped2d::b2;

const BODY_SKIN_DEPTH: f32 = 0.02;
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
        out: BlendTarget<gfx::format::Srgba8> = ("o_Color",
                                                 gfx::state::ColorMask::all(),
                                                 gfx::preset::blend::ALPHA),
    }
}

#[derive(Debug)]
struct Boundary {
    body_handle: b2::BodyHandle,
    vertices: Vec<b2::Vec2>,
}

impl Boundary {
    fn new(world: &mut World, vertices: &[b2::Vec2]) -> Self {
        let handle = world.create_body(&b2::BodyDef::new());
        let mut body = world.body_mut(handle);
        body.create_fast_fixture(&b2::ChainShape::new_chain(vertices), 1.0);
        Boundary {
            body_handle: handle,
            vertices: vertices.to_vec(),
        }
    }

    fn extend_vertex_buffer(&self, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
        for i in 1..self.vertices.len() {
            let start = vertices.len() as u32;
            let (from, to) = (self.vertices[i - 1], self.vertices[i]);
            vertices.extend(&[Vertex {
                                  pos: [to.x + BODY_SKIN_DEPTH, to.y + BODY_SKIN_DEPTH],
                                  uv: [0.5, 0.5],
                                  color: color::BLACK,
                              },
                              Vertex {
                                  pos: [from.x - BODY_SKIN_DEPTH,
                                        from.y + BODY_SKIN_DEPTH],
                                  uv: [0.5, 0.5],
                                  color: color::BLACK,
                              },
                              Vertex {
                                  pos: [from.x - BODY_SKIN_DEPTH, 0.0],
                                  uv: [0.5, 0.5],
                                  color: color::BLACK,
                              },
                              Vertex {
                                  pos: [to.x + BODY_SKIN_DEPTH, 0.0],
                                  uv: [0.5, 0.5],
                                  color: color::BLACK,
                              }]);
            indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start]);
        }
    }
}

#[derive(Debug)]
struct Token {
    body_handle: b2::BodyHandle,
    radius: f32,
    color: Color,
}

impl Token {
    fn new(world: &mut World, x: f32, y: f32, radius: f32, color: Color) -> Self {
        let handle = world.create_body(&b2::BodyDef {
                                           body_type: b2::BodyType::Dynamic,
                                           position: b2::Vec2 { x: x, y: y },
                                           ..b2::BodyDef::new()
                                       });
        let mut body = world.body_mut(handle);
        let mut shape = b2::CircleShape::new();
        shape.set_radius(radius);
        body.create_fixture(&shape,
                            &mut b2::FixtureDef {
                                density: 1.0,
                                friction: 0.666,
                                restitution: 0.42,
                                ..b2::FixtureDef::new()
                            });
        Token {
            body_handle: handle,
            radius: radius,
            color: color,
        }
    }

    fn survives(&self, world: &mut World) -> bool {
        if world.body(self.body_handle).position().y < -2.0 {
            world.destroy_body(self.body_handle);
            false
        } else {
            true
        }
    }

    fn extend_vertex_buffer(&self,
                            world: &World,
                            vertices: &mut Vec<Vertex>,
                            indices: &mut Vec<u32>) {
        const THICKNESS: f32 = 0.042;
        let start = vertices.len() as u32;
        let body = world.body(self.body_handle);
        let transform = body.transform();
        let radius = self.radius + BODY_SKIN_DEPTH;
        let inner_radius = self.radius - THICKNESS * 2.0;
        vertices.extend(&[Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: radius,
                                         y: radius,
                                     }).as_array(),
                              uv: [1.0, 0.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -radius,
                                         y: radius,
                                     }).as_array(),
                              uv: [0.0, 0.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -radius,
                                         y: -radius,
                                     }).as_array(),
                              uv: [0.0, 1.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: radius,
                                         y: -radius,
                                     }).as_array(),
                              uv: [1.0, 1.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: inner_radius,
                                         y: inner_radius,
                                     }).as_array(),
                              uv: [1.0, 0.0],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -inner_radius,
                                         y: inner_radius,
                                     }).as_array(),
                              uv: [0.0, 0.0],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -inner_radius,
                                         y: -inner_radius,
                                     }).as_array(),
                              uv: [0.0, 1.0],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: inner_radius,
                                         y: -inner_radius,
                                     }).as_array(),
                              uv: [1.0, 1.0],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: THICKNESS,
                                         y: self.radius,
                                     }).as_array(),
                              uv: [0.5, 0.5],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -THICKNESS,
                                         y: self.radius,
                                     }).as_array(),
                              uv: [0.5, 0.5],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -THICKNESS,
                                         y: -THICKNESS,
                                     }).as_array(),
                              uv: [0.5, 0.5],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: THICKNESS,
                                         y: -THICKNESS,
                                     }).as_array(),
                              uv: [0.5, 0.5],
                              color: color::BLACK,
                          }]);
        indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start,
                         start + 4, start + 5, start + 6, start + 6, start + 7,
                         start + 4, start + 8, start + 9, start + 10, start + 10,
                         start + 11, start + 8]);
    }
}

struct App {
    world: World,
    boundary: Option<Boundary>,
    tokens: Vec<Token>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    pipeline: Option<PistonPipeline<world::Meta>>,
    renderer: Option<PistonRenderer>,
}

impl App {
    fn new() -> Self {
        const GRAVITY: b2::Vec2 = b2::Vec2 { x: 0.0, y: -10.0 };
        App {
            world: World::new(&GRAVITY),
            boundary: None,
            tokens: vec![],
            vertices: Vec::with_capacity(4 * 4096),
            indices: Vec::with_capacity(6 * 4096),
            pipeline: None,
            renderer: None,
        }
    }

    fn dump_data(&self, state: &PistonAppState) {
        let token_count = self.tokens.len();
        let vertex_count = self.vertices.len();
        let index_count = self.indices.len();
        let memory = (std::mem::size_of::<Boundary>() +
                      token_count * std::mem::size_of::<Token>() +
                      vertex_count * std::mem::size_of::<Vertex>() +
                      index_count * std::mem::size_of::<u32>()) as
                     f32 / 1024.0;
        println!("Frame {} | Tokens: {} | Vertices: {} | Indices: {} | Memory: {:.2} KB",
                 state.frame_count(),
                 token_count,
                 vertex_count,
                 index_count,
                 memory);
    }

    fn setup_world(&mut self, state: &PistonAppState) {
        let width = state.width() as f32;
        let half_width = width / 2.0 / PIXELS_PER_METER + 1.0;
        self.boundary = Some(Boundary::new(&mut self.world,
                                           &[b2::Vec2 {
                                                 x: -half_width,
                                                 y: 2.0,
                                             },
                                             b2::Vec2 { x: 0.0, y: 2.0 },
                                             b2::Vec2 {
                                                 x: half_width,
                                                 y: 4.0,
                                             }]));
    }

    fn spawn_token(&mut self, state: &PistonAppState) {
        let x = (state.mouse_x() - state.width() / 2.0) as f32 / PIXELS_PER_METER;
        let y = (state.height() - state.mouse_y()) as f32 / PIXELS_PER_METER;
        let token = Token::new(&mut self.world,
                               x,
                               y,
                               thread_rng().gen_range(0.16, 0.5),
                               state.random_color(Some(1.0)));
        self.tokens.push(token);
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.setup_world(state);
        let (pipeline, renderer) = PistonPipelineBuilder::new()
            .texture_atlas(TextureAtlas::from_path(window, "assets/token.png").unwrap())
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
            self.spawn_token(state);
        }
        self.vertices.clear();
        self.indices.clear();
        let world = &mut self.world;
        world.step(1.0 / 60.0, 8, 3);
        world.clear_forces();
        self.tokens.retain(|token| token.survives(world));
        for token in &self.tokens {
            token.extend_vertex_buffer(world, &mut self.vertices, &mut self.indices);
        }
        let boundary = self.boundary.as_ref().unwrap();
        boundary.extend_vertex_buffer(&mut self.vertices, &mut self.indices);
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

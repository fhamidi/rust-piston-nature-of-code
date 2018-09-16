//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Physics libraries - Complex bodies.

extern crate piston_app;
extern crate wrapped2d;

use piston_app::*;
use wrapped2d::b2;

const BODY_DELTA: f32 = 0.3;
const BODY_HALF_WIDTH: f32 = 0.125;
const BODY_HALF_HEIGHT: f32 = 0.5;
const BODY_RADIUS: f32 = 0.25;
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

    fn extend_vertex_buffer(&self,
                            texture_atlas: &TextureAtlas,
                            vertices: &mut Vec<Vertex>,
                            indices: &mut Vec<u32>) {
        let start = vertices.len() as u32;
        let (x, y) = (self.x, self.y);
        let (w, h) = (self.half_width + BODY_SKIN_DEPTH,
                      self.half_height + BODY_SKIN_DEPTH);
        let (u, v, tw, th) = texture_atlas.texture_uv_extents(0);
        vertices.extend(&[Vertex {
                              pos: [x + w, y + h],
                              uv: [u + tw, v],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [x - w, y + h],
                              uv: [u, v],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [x - w, y - h],
                              uv: [u, v + th],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [x + w, y - h],
                              uv: [u + tw, v + th],
                              color: color::BLACK,
                          }]);
        indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start]);
    }
}

#[derive(Debug)]
struct Entity {
    body_handle: b2::BodyHandle,
    color: Color,
}

impl Entity {
    fn new(world: &mut World, x: f32, y: f32, color: Color) -> Self {
        let handle = world.create_body(&b2::BodyDef {
                                           body_type: b2::BodyType::Dynamic,
                                           position: b2::Vec2 { x: x, y: y },
                                           ..b2::BodyDef::new()
                                       });
        let mut body = world.body_mut(handle);
        body.create_fast_fixture(&b2::PolygonShape::new_box(BODY_HALF_WIDTH,
                                                            BODY_HALF_HEIGHT),
                                 1.0);
        body.create_fast_fixture(&b2::CircleShape::new_with(b2::Vec2 {
                                                                x: 0.0,
                                                                y: BODY_DELTA,
                                                            },
                                                            BODY_RADIUS),
                                 1.0);
        Entity {
            body_handle: handle,
            color: color,
        }
    }

    fn extend_vertex_buffer(&self,
                            world: &World,
                            texture_atlas: &TextureAtlas,
                            vertices: &mut Vec<Vertex>,
                            indices: &mut Vec<u32>) {
        const THICKNESS: f32 = 0.084;
        let start = vertices.len() as u32;
        let body = world.body(self.body_handle);
        let transform = body.transform();
        let w = BODY_HALF_WIDTH + BODY_SKIN_DEPTH;
        let h = BODY_HALF_HEIGHT + BODY_SKIN_DEPTH;
        let r = BODY_RADIUS + BODY_SKIN_DEPTH;
        let (iw, ih, ir) = (w - THICKNESS, h - THICKNESS, r - THICKNESS);
        let coords = [transform * b2::Vec2 { x: w, y: h },
                      transform * b2::Vec2 { x: -w, y: h },
                      transform * b2::Vec2 { x: -w, y: -h },
                      transform * b2::Vec2 { x: w, y: -h },
                      transform * b2::Vec2 { x: iw, y: ih },
                      transform * b2::Vec2 { x: -iw, y: ih },
                      transform * b2::Vec2 { x: -iw, y: -ih },
                      transform * b2::Vec2 { x: iw, y: -ih },
                      transform *
                      b2::Vec2 {
                          x: r,
                          y: r + BODY_DELTA,
                      },
                      transform *
                      b2::Vec2 {
                          x: -r,
                          y: r + BODY_DELTA,
                      },
                      transform *
                      b2::Vec2 {
                          x: -r,
                          y: -r + BODY_DELTA,
                      },
                      transform *
                      b2::Vec2 {
                          x: r,
                          y: -r + BODY_DELTA,
                      },
                      transform *
                      b2::Vec2 {
                          x: ir,
                          y: ir + BODY_DELTA,
                      },
                      transform *
                      b2::Vec2 {
                          x: -ir,
                          y: ir + BODY_DELTA,
                      },
                      transform *
                      b2::Vec2 {
                          x: -ir,
                          y: -ir + BODY_DELTA,
                      },
                      transform *
                      b2::Vec2 {
                          x: ir,
                          y: -ir + BODY_DELTA,
                      }];
        let (u, v, tw, th) = texture_atlas.texture_uv_extents(0);
        let (ru, rv, rw, rh) = texture_atlas.texture_uv_extents(1);
        vertices.extend(&[Vertex {
                              pos: [coords[0].x, coords[0].y],
                              uv: [u + tw, v],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [coords[1].x, coords[1].y],
                              uv: [u, v],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [coords[2].x, coords[2].y],
                              uv: [u, v + th],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [coords[3].x, coords[3].y],
                              uv: [u + tw, v + th],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [coords[4].x, coords[4].y],
                              uv: [u + tw, v],
                              color: self.color,
                          },
                          Vertex {
                              pos: [coords[5].x, coords[5].y],
                              uv: [u, v],
                              color: self.color,
                          },
                          Vertex {
                              pos: [coords[6].x, coords[6].y],
                              uv: [u, v + th],
                              color: self.color,
                          },
                          Vertex {
                              pos: [coords[7].x, coords[7].y],
                              uv: [u + tw, v + th],
                              color: self.color,
                          },
                          Vertex {
                              pos: [coords[8].x, coords[8].y],
                              uv: [ru + rw, rv],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [coords[9].x, coords[9].y],
                              uv: [ru, rv],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [coords[10].x, coords[10].y],
                              uv: [ru, rv + rh],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [coords[11].x, coords[11].y],
                              uv: [ru + rw, rv + rh],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: [coords[12].x, coords[12].y],
                              uv: [ru + rw, rv],
                              color: self.color,
                          },
                          Vertex {
                              pos: [coords[13].x, coords[13].y],
                              uv: [ru, rv],
                              color: self.color,
                          },
                          Vertex {
                              pos: [coords[14].x, coords[14].y],
                              uv: [ru, rv + rh],
                              color: self.color,
                          },
                          Vertex {
                              pos: [coords[15].x, coords[15].y],
                              uv: [ru + rw, rv + rh],
                              color: self.color,
                          }]);
        indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start,
                         start + 4, start + 5, start + 6, start + 6, start + 7,
                         start + 4, start + 8, start + 9, start + 10, start + 10,
                         start + 11, start + 8, start + 12, start + 13, start + 14,
                         start + 14, start + 15, start + 12]);
    }
}

struct App {
    world: Option<World>,
    boundaries: Vec<Boundary>,
    entities: Vec<Entity>,
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
            entities: vec![],
            vertices: Vec::with_capacity(4 * 4096),
            indices: Vec::with_capacity(6 * 4096),
            pipeline: None,
            renderer: None,
        }
    }

    fn dump_data(&self, state: &PistonAppState) {
        let boundary_count = self.boundaries.len();
        let entity_count = self.entities.len();
        let vertex_count = self.vertices.len();
        let index_count = self.indices.len();
        let memory = (boundary_count * std::mem::size_of::<Boundary>() +
                      entity_count * std::mem::size_of::<Entity>() +
                      vertex_count * std::mem::size_of::<Vertex>() +
                      index_count * std::mem::size_of::<u32>()) as
                     f32 / 1024.0;
        println!("Frame {} | Entities: {} | Vertices: {} | Indices: {} | Memory: {:.2} KB",
                 state.frame_count(),
                 entity_count,
                 vertex_count,
                 index_count,
                 memory);
    }

    fn setup_world(&mut self, state: &PistonAppState) {
        const MAX_BOUNDARIES: usize = 5;
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
        self.boundaries = (0..MAX_BOUNDARIES)
            .map(|i| {
                let side = if i % 2 == 0 { -1.0 } else { 1.0 };
                Boundary::new(&mut world,
                              (boundary_width / 2.0 + 1.0) * side,
                              (i + 1) as f32 * 2.0,
                              boundary_width,
                              0.5)
            })
            .collect();
        self.world = Some(world);
    }

    fn spawn_entity(&mut self, state: &PistonAppState) {
        let x = (state.mouse_x() - state.width() / 2.0) as f32 / PIXELS_PER_METER;
        let y = (state.height() - state.mouse_y()) as f32 / PIXELS_PER_METER;
        let entity = Entity::new(self.world.as_mut().unwrap(),
                                 x,
                                 y,
                                 state.random_color(Some(1.0)));
        self.entities.push(entity);
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.setup_world(state);
        let (pipeline, renderer) = PistonPipelineBuilder::new()
            .texture_atlas(TextureAtlas::from_paths(window,
                                                    "assets/shapes.png",
                                                    "assets/shapes.atlas")
                               .unwrap())
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
            self.spawn_entity(state);
        }
        self.vertices.clear();
        self.indices.clear();
        let world = self.world.as_mut().unwrap();
        world.step(1.0 / 60.0, 8, 3);
        world.clear_forces();
        let renderer = self.renderer.as_ref().unwrap();
        let texture_atlas = renderer.texture_atlas().unwrap();
        for entity in &self.entities {
            entity.extend_vertex_buffer(world,
                                        texture_atlas,
                                        &mut self.vertices,
                                        &mut self.indices);
        }
        for boundary in &self.boundaries {
            boundary.extend_vertex_buffer(texture_atlas,
                                          &mut self.vertices,
                                          &mut self.indices);
        }
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

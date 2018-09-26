//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Physics libraries - Composite entities using jointed bodies.

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
        out: BlendTarget<gfx::format::Srgba8> = ("o_color",
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
        body.create_fast_fixture(&b2::PolygonShape::new_box(half_width, half_height),
                                 0.5);
        Brick {
            body_handle: handle,
            half_width: half_width,
            half_height: half_height,
            color: color,
        }
    }

    fn body_handle(&self) -> b2::BodyHandle {
        self.body_handle
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
        let (w, h) = (self.half_width + BODY_SKIN_DEPTH,
                      self.half_height + BODY_SKIN_DEPTH);
        let (iw, ih) = (w - THICKNESS, h - THICKNESS);
        let (u, v, tw, th) = texture_atlas.texture_uv_extents(0);
        vertices.extend(&[Vertex {
                              pos: *(transform * b2::Vec2 { x: w, y: h }).as_array(),
                              uv: [u + tw, v],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: -w, y: h }).as_array(),
                              uv: [u, v],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: -w, y: -h }).as_array(),
                              uv: [u, v + th],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: w, y: -h }).as_array(),
                              uv: [u + tw, v + th],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: iw, y: ih }).as_array(),
                              uv: [u + tw, v],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: -iw, y: ih }).as_array(),
                              uv: [u, v],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: -iw, y: -ih }).as_array(),
                              uv: [u, v + th],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform * b2::Vec2 { x: iw, y: -ih }).as_array(),
                              uv: [u + tw, v + th],
                              color: self.color,
                          }]);
        indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start,
                         start + 4, start + 5, start + 6, start + 6, start + 7,
                         start + 4]);
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
                                restitution: 0.42,
                                ..b2::FixtureDef::new()
                            });
        Token {
            body_handle: handle,
            radius: radius,
            color: color,
        }
    }

    fn body_handle(&self) -> b2::BodyHandle {
        self.body_handle
    }

    fn extend_vertex_buffer(&self,
                            world: &World,
                            texture_atlas: &TextureAtlas,
                            vertices: &mut Vec<Vertex>,
                            indices: &mut Vec<u32>) {
        const THICKNESS: f32 = 0.042;
        let start = vertices.len() as u32;
        let body = world.body(self.body_handle);
        let transform = body.transform();
        let radius = self.radius + BODY_SKIN_DEPTH;
        let inner_radius = self.radius - THICKNESS * 2.0;
        let (u, v, tw, th) = texture_atlas.texture_uv_extents(1);
        vertices.extend(&[Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: radius,
                                         y: radius,
                                     }).as_array(),
                              uv: [u + tw, v],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -radius,
                                         y: radius,
                                     }).as_array(),
                              uv: [u, v],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -radius,
                                         y: -radius,
                                     }).as_array(),
                              uv: [u, v + th],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: radius,
                                         y: -radius,
                                     }).as_array(),
                              uv: [u + tw, v + th],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: inner_radius,
                                         y: inner_radius,
                                     }).as_array(),
                              uv: [u + tw, v],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -inner_radius,
                                         y: inner_radius,
                                     }).as_array(),
                              uv: [u, v],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -inner_radius,
                                         y: -inner_radius,
                                     }).as_array(),
                              uv: [u, v + th],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: inner_radius,
                                         y: -inner_radius,
                                     }).as_array(),
                              uv: [u + tw, v + th],
                              color: self.color,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: THICKNESS,
                                         y: self.radius,
                                     }).as_array(),
                              uv: [u + tw / 2.0, v + th / 2.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -THICKNESS,
                                         y: self.radius,
                                     }).as_array(),
                              uv: [u + tw / 2.0, v + th / 2.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: -THICKNESS,
                                         y: -THICKNESS,
                                     }).as_array(),
                              uv: [u + tw / 2.0, v + th / 2.0],
                              color: color::BLACK,
                          },
                          Vertex {
                              pos: *(transform *
                                     b2::Vec2 {
                                         x: THICKNESS,
                                         y: -THICKNESS,
                                     }).as_array(),
                              uv: [u + tw / 2.0, v + th / 2.0],
                              color: color::BLACK,
                          }]);
        indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start,
                         start + 4, start + 5, start + 6, start + 6, start + 7,
                         start + 4, start + 8, start + 9, start + 10, start + 10,
                         start + 11, start + 8]);
    }
}

#[derive(Debug)]
struct Entity {
    brick: Brick,
    tokens: [Token; 2],
}

const BRICK_WIDTH: f32 = 0.25;
const BRICK_HEIGHT: f32 = 0.84;
const ANCHOR_DELTA: f32 = BRICK_HEIGHT / 2.0 - 0.24;
const TOKEN_DELTA: f32 = 1.25;
const TOKEN_RADIUS: f32 = 0.32;

impl Entity {
    fn new(world: &mut World, x: f32, y: f32, color: Color) -> Self {
        let brick = Brick::new(world, x, y, BRICK_WIDTH, BRICK_HEIGHT, color);
        let tokens = [Token::new(world, x, y + TOKEN_DELTA, TOKEN_RADIUS, color),
                      Token::new(world, x, y - TOKEN_DELTA, TOKEN_RADIUS, color)];
        world.create_joint(&b2::DistanceJointDef {
                               collide_connected: true,
                               damping_ratio: 0.24,
                               frequency: 4.2,
                               length: TOKEN_DELTA - ANCHOR_DELTA,
                               local_anchor_a: b2::Vec2 {
                                   x: 0.0,
                                   y: ANCHOR_DELTA,
                               },
                               ..b2::DistanceJointDef::new(brick.body_handle(),
                                                           tokens[0].body_handle())
                           });
        world.create_joint(&b2::DistanceJointDef {
                               collide_connected: true,
                               damping_ratio: 0.24,
                               frequency: 4.2,
                               length: TOKEN_DELTA - ANCHOR_DELTA,
                               local_anchor_a: b2::Vec2 {
                                   x: 0.0,
                                   y: -ANCHOR_DELTA,
                               },
                               ..b2::DistanceJointDef::new(brick.body_handle(),
                                                           tokens[1].body_handle())
                           });
        Entity {
            brick: brick,
            tokens: tokens,
        }
    }

    fn extend_vertex_buffer(&self,
                            world: &World,
                            texture_atlas: &TextureAtlas,
                            vertices: &mut Vec<Vertex>,
                            indices: &mut Vec<u32>) {
        self.render_joints(world, texture_atlas, vertices, indices);
        self.brick
            .extend_vertex_buffer(world, texture_atlas, vertices, indices);
        for token in &self.tokens {
            token.extend_vertex_buffer(world, texture_atlas, vertices, indices);
        }
    }

    fn render_joints(&self,
                     world: &World,
                     texture_atlas: &TextureAtlas,
                     vertices: &mut Vec<Vertex>,
                     indices: &mut Vec<u32>) {
        const THICKNESS: f32 = 0.042;
        let start = vertices.len() as u32;
        let brick_body = world.body(self.brick.body_handle());
        let brick_transform = brick_body.transform();
        let token_bodies = [world.body(self.tokens[0].body_handle()),
                            world.body(self.tokens[1].body_handle())];
        let token_transforms = [token_bodies[0].transform(), token_bodies[1].transform()];
        let mut coords: Vec<b2::Vec2> = vec![];
        coords.extend(&self.compute_line_coords(brick_transform *
                                                b2::Vec2 {
                                                    x: 0.0,
                                                    y: ANCHOR_DELTA,
                                                },
                                                token_transforms[0] *
                                                b2::Vec2 { x: 0.0, y: 0.0 },
                                                THICKNESS));
        coords.extend(&self.compute_line_coords(brick_transform *
                                                b2::Vec2 {
                                                    x: 0.0,
                                                    y: -ANCHOR_DELTA,
                                                },
                                                token_transforms[1] *
                                                b2::Vec2 { x: 0.0, y: 0.0 },
                                                THICKNESS));
        let (u, v, tw, th) = texture_atlas.texture_uv_extents(0);
        let uv = [u + tw / 2.0, v + th / 2.0];
        let color = color::grey(0.25);
        vertices.extend(&[Vertex {
                              pos: *coords[0].as_array(),
                              uv: uv,
                              color: color,
                          },
                          Vertex {
                              pos: *coords[1].as_array(),
                              uv: uv,
                              color: color,
                          },
                          Vertex {
                              pos: *coords[2].as_array(),
                              uv: uv,
                              color: color,
                          },
                          Vertex {
                              pos: *coords[3].as_array(),
                              uv: uv,
                              color: color,
                          },
                          Vertex {
                              pos: *coords[4].as_array(),
                              uv: uv,
                              color: color,
                          },
                          Vertex {
                              pos: *coords[5].as_array(),
                              uv: uv,
                              color: color,
                          },
                          Vertex {
                              pos: *coords[6].as_array(),
                              uv: uv,
                              color: color,
                          },
                          Vertex {
                              pos: *coords[7].as_array(),
                              uv: uv,
                              color: color,
                          }]);
        indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start,
                         start + 4, start + 5, start + 6, start + 6, start + 7,
                         start + 4]);
    }

    fn compute_line_coords(&self,
                           start: b2::Vec2,
                           end: b2::Vec2,
                           thickness: f32)
                           -> [b2::Vec2; 4] {
        let normal = (end - start).sqew();
        let delta = normal * thickness / normal.norm();
        [end + delta, start + delta, start - delta, end - delta]
    }
}

struct App {
    world: World,
    boundaries: Vec<Boundary>,
    entities: Vec<Entity>,
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
        const MAX_BOUNDARIES: usize = 3;
        let ground = self.world.create_body(&b2::BodyDef {
                                                position: b2::Vec2 { x: 0.0, y: -10.0 },
                                                ..b2::BodyDef::new()
                                            });
        let width = state.width() as f32;
        let shape = b2::PolygonShape::new_box(width * 4.2 / PIXELS_PER_METER, 10.0);
        self.world.body_mut(ground).create_fast_fixture(&shape, 0.0);
        let boundary_width = width / 2.0 / PIXELS_PER_METER - 2.0;
        self.boundaries = (0..MAX_BOUNDARIES)
            .map(|i| {
                     let side = if i % 2 == 0 { -1.0 } else { 1.0 };
                     Boundary::new(&mut self.world,
                                   (boundary_width / 2.0 + 1.0) * side,
                                   (i + 1) as f32 * 3.2 - 1.2,
                                   boundary_width,
                                   0.5)
                 })
            .collect();
    }

    fn spawn_entity(&mut self, state: &PistonAppState) {
        let x = (state.mouse_x() - state.width() / 2.0) as f32 / PIXELS_PER_METER;
        let y = (state.height() - state.mouse_y()) as f32 / PIXELS_PER_METER;
        let entity = Entity::new(&mut self.world, x, y, state.random_color(Some(1.0)));
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
        self.world.step(1.0 / 60.0, 8, 3);
        self.world.clear_forces();
        let renderer = self.renderer.as_ref().unwrap();
        let texture_atlas = renderer.texture_atlas().unwrap();
        for entity in &self.entities {
            entity.extend_vertex_buffer(&self.world,
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

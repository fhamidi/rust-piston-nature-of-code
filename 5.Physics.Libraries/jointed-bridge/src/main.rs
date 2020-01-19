//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Physics libraries - Jointed bridge.

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
struct Brick {
    body_handle: b2::BodyHandle,
    half_width: f32,
    half_height: f32,
    color: Color,
}

impl Brick {
    fn new(
        world: &mut World,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
    ) -> Self {
        let handle = world.create_body(&b2::BodyDef {
            body_type: b2::BodyType::Dynamic,
            position: b2::Vec2 { x: x, y: y },
            ..b2::BodyDef::new()
        });
        let mut body = world.body_mut(handle);
        let (half_width, half_height) = (width / 2.0, height / 2.0);
        body.create_fixture(
            &b2::PolygonShape::new_box(half_width, half_height),
            &mut b2::FixtureDef {
                density: 1.0,
                friction: 0.3,
                restitution: 0.5,
                ..b2::FixtureDef::new()
            },
        );
        Brick {
            body_handle: handle,
            half_width: half_width,
            half_height: half_height,
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

    fn extend_vertex_buffer(
        &self,
        world: &World,
        texture_atlas: &TextureAtlas,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
    ) {
        const THICKNESS: f32 = 0.084;
        let start = vertices.len() as u32;
        let body = world.body(self.body_handle);
        let transform = body.transform();
        let (w, h) = (
            self.half_width + BODY_SKIN_DEPTH,
            self.half_height + BODY_SKIN_DEPTH,
        );
        let (iw, ih) = (w - THICKNESS, h - THICKNESS);
        let (u, v, tw, th) = texture_atlas.texture_uv_extents(0);
        vertices.extend(&[
            Vertex {
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
            },
        ]);
        indices.extend(&[
            start,
            start + 1,
            start + 2,
            start + 2,
            start + 3,
            start,
            start + 4,
            start + 5,
            start + 6,
            start + 6,
            start + 7,
            start + 4,
        ]);
    }
}

#[derive(Debug)]
struct Token {
    body_handle: b2::BodyHandle,
    radius: f32,
}

impl Token {
    fn new(world: &mut World, x: f32, y: f32, radius: f32, density: f32) -> Self {
        let body_type = if density > 0.0 {
            b2::BodyType::Dynamic
        } else {
            b2::BodyType::Static
        };
        let handle = world.create_body(&b2::BodyDef {
            body_type: body_type,
            position: b2::Vec2 { x: x, y: y },
            ..b2::BodyDef::new()
        });
        let mut body = world.body_mut(handle);
        let mut shape = b2::CircleShape::new();
        shape.set_radius(radius);
        body.create_fixture(
            &shape,
            &mut b2::FixtureDef {
                density: density,
                restitution: 0.42,
                ..b2::FixtureDef::new()
            },
        );
        Token {
            body_handle: handle,
            radius: radius,
        }
    }

    fn body_handle(&self) -> b2::BodyHandle {
        self.body_handle
    }

    fn extend_vertex_buffer(
        &self,
        world: &World,
        texture_atlas: &TextureAtlas,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
    ) {
        const THICKNESS: f32 = 0.042;
        let start = vertices.len() as u32;
        let body = world.body(self.body_handle);
        let transform = body.transform();
        let radius = self.radius + BODY_SKIN_DEPTH;
        let inner_radius = self.radius - THICKNESS * 2.0;
        let (u, v, tw, th) = texture_atlas.texture_uv_extents(1);
        let color = color::grey(0.25);
        vertices.extend(&[
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: radius,
                        y: radius,
                    })
                .as_array(),
                uv: [u + tw, v],
                color: color::BLACK,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: -radius,
                        y: radius,
                    })
                .as_array(),
                uv: [u, v],
                color: color::BLACK,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: -radius,
                        y: -radius,
                    })
                .as_array(),
                uv: [u, v + th],
                color: color::BLACK,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: radius,
                        y: -radius,
                    })
                .as_array(),
                uv: [u + tw, v + th],
                color: color::BLACK,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: inner_radius,
                        y: inner_radius,
                    })
                .as_array(),
                uv: [u + tw, v],
                color: color,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: -inner_radius,
                        y: inner_radius,
                    })
                .as_array(),
                uv: [u, v],
                color: color,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: -inner_radius,
                        y: -inner_radius,
                    })
                .as_array(),
                uv: [u, v + th],
                color: color,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: inner_radius,
                        y: -inner_radius,
                    })
                .as_array(),
                uv: [u + tw, v + th],
                color: color,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: THICKNESS,
                        y: self.radius,
                    })
                .as_array(),
                uv: [u + tw / 2.0, v + th / 2.0],
                color: color::BLACK,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: -THICKNESS,
                        y: self.radius,
                    })
                .as_array(),
                uv: [u + tw / 2.0, v + th / 2.0],
                color: color::BLACK,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: -THICKNESS,
                        y: -THICKNESS,
                    })
                .as_array(),
                uv: [u + tw / 2.0, v + th / 2.0],
                color: color::BLACK,
            },
            Vertex {
                pos: *(transform
                    * b2::Vec2 {
                        x: THICKNESS,
                        y: -THICKNESS,
                    })
                .as_array(),
                uv: [u + tw / 2.0, v + th / 2.0],
                color: color::BLACK,
            },
        ]);
        indices.extend(&[
            start,
            start + 1,
            start + 2,
            start + 2,
            start + 3,
            start,
            start + 4,
            start + 5,
            start + 6,
            start + 6,
            start + 7,
            start + 4,
            start + 8,
            start + 9,
            start + 10,
            start + 10,
            start + 11,
            start + 8,
        ]);
    }
}

#[derive(Debug)]
struct Bridge {
    tokens: Vec<Token>,
}

impl Bridge {
    fn new(world: &mut World, x: f32, y: f32, width: f32) -> Self {
        const STEP: f32 = 0.5;
        let count = (width / STEP) as usize + 1;
        let tokens: Vec<Token> = (0..count)
            .map(|i| {
                let density = if i == 0 || i == count - 1 { 0.0 } else { 1.0 };
                Token::new(world, x + i as f32 * STEP, y, 0.25, density)
            })
            .collect();
        for i in 1..count {
            world.create_joint(&b2::DistanceJointDef {
                collide_connected: true,
                damping_ratio: 0.42,
                length: STEP,
                ..b2::DistanceJointDef::new(
                    tokens[i - 1].body_handle(),
                    tokens[i].body_handle(),
                )
            });
        }
        Bridge { tokens: tokens }
    }

    fn extend_vertex_buffer(
        &self,
        world: &World,
        texture_atlas: &TextureAtlas,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
    ) {
        self.render_joints(world, texture_atlas, vertices, indices);
        for token in &self.tokens {
            token.extend_vertex_buffer(world, texture_atlas, vertices, indices);
        }
    }

    fn render_joints(
        &self,
        world: &World,
        texture_atlas: &TextureAtlas,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
    ) {
        const THICKNESS: f32 = 0.042;
        let (u, v, tw, th) = texture_atlas.texture_uv_extents(0);
        let uv = [u + tw / 2.0, v + th / 2.0];
        let color = color::grey(0.25);
        for i in 1..self.tokens.len() {
            let start = vertices.len() as u32;
            let bodies = [
                world.body(self.tokens[i - 1].body_handle()),
                world.body(self.tokens[i].body_handle()),
            ];
            let transforms = [bodies[0].transform(), bodies[1].transform()];
            let first = transforms[0] * b2::Vec2 { x: 0.0, y: 0.0 };
            let second = transforms[1] * b2::Vec2 { x: 0.0, y: 0.0 };
            let normal = (second - first).sqew();
            let delta = normal * THICKNESS / normal.norm();
            vertices.extend(&[
                Vertex {
                    pos: *(second + delta).as_array(),
                    uv: uv,
                    color: color,
                },
                Vertex {
                    pos: *(first + delta).as_array(),
                    uv: uv,
                    color: color,
                },
                Vertex {
                    pos: *(first - delta).as_array(),
                    uv: uv,
                    color: color,
                },
                Vertex {
                    pos: *(second - delta).as_array(),
                    uv: uv,
                    color: color,
                },
            ]);
            indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start]);
        }
    }
}

struct App {
    world: World,
    bridge: Option<Bridge>,
    bricks: Vec<Brick>,
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
            bridge: None,
            bricks: vec![],
            vertices: Vec::with_capacity(4 * 4096),
            indices: Vec::with_capacity(6 * 4096),
            pipeline: None,
            renderer: None,
        }
    }

    fn dump_data(&self, state: &PistonAppState) {
        let brick_count = self.bricks.len();
        let vertex_count = self.vertices.len();
        let index_count = self.indices.len();
        let memory = (brick_count * std::mem::size_of::<Brick>()
            + vertex_count * std::mem::size_of::<Vertex>()
            + index_count * std::mem::size_of::<u32>()) as f32
            / 1024.0;
        println!(
            "Frame {} | Bricks: {} | Vertices: {} | Indices: {} | Memory: {:.2} KB",
            state.frame_count(),
            brick_count,
            vertex_count,
            index_count,
            memory
        );
    }

    fn setup_world(&mut self, state: &PistonAppState) {
        let width = state.width() as f32 / PIXELS_PER_METER;
        self.bridge = Some(Bridge::new(&mut self.world, -width / 2.0, 5.0, width));
    }

    fn spawn_brick(&mut self, state: &PistonAppState) {
        let mut rng = thread_rng();
        let uniform = Uniform::new_inclusive(0.2, 1.0);
        let x = (state.mouse_x() - state.width() / 2.0) as f32 / PIXELS_PER_METER;
        let y = (state.height() - state.mouse_y()) as f32 / PIXELS_PER_METER;
        let brick = Brick::new(
            &mut self.world,
            x,
            y,
            rng.sample(uniform),
            rng.sample(uniform),
            state.random_color(Some(1.0)),
        );
        self.bricks.push(brick);
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.setup_world(state);
        let (pipeline, renderer) = PistonPipelineBuilder::new()
            .texture_atlas(
                TextureAtlas::from_paths(
                    window,
                    "assets/shapes.png",
                    "assets/shapes.atlas",
                )
                .unwrap(),
            )
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
        let world = &mut self.world;
        world.step(1.0 / 60.0, 8, 3);
        world.clear_forces();
        self.bricks.retain(|brick| brick.survives(world));
        let renderer = self.renderer.as_ref().unwrap();
        let texture_atlas = renderer.texture_atlas().unwrap();
        self.bridge.as_ref().unwrap().extend_vertex_buffer(
            world,
            texture_atlas,
            &mut self.vertices,
            &mut self.indices,
        );
        for brick in &self.bricks {
            brick.extend_vertex_buffer(
                world,
                texture_atlas,
                &mut self.vertices,
                &mut self.indices,
            );
        }
        let half_width = state.width() as f32 / 2.0;
        let half_height = state.height() as f32 / 2.0;
        renderer.clear(window, color::WHITE);
        renderer.draw(
            window,
            self.pipeline.as_ref().unwrap(),
            &self.vertices[..],
            &self.indices[..],
            |vbuf, out| world::Data {
                vbuf: vbuf,
                sampler: texture_atlas.texture_view_sampler(),
                transform: [
                    0.0,
                    -1.0,
                    PIXELS_PER_METER / half_width,
                    PIXELS_PER_METER / half_height,
                ],
                out: out,
            },
        );
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

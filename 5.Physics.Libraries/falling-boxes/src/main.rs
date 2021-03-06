//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Physics libraries - Falling boxes.

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
        out: RenderTarget<gfx::format::Srgba8> = "o_color",
    }
}

#[derive(Debug)]
struct FallingBox {
    body_handle: b2::BodyHandle,
    color: Color,
}

impl FallingBox {
    fn new(world: &mut World, x: f32, y: f32, color: Color) -> Self {
        let handle = world.create_body(&b2::BodyDef {
            body_type: b2::BodyType::Dynamic,
            position: b2::Vec2 { x: x, y: y },
            ..b2::BodyDef::new()
        });
        let mut body = world.body_mut(handle);
        body.create_fixture(
            &b2::PolygonShape::new_box(0.5, 0.5),
            &mut b2::FixtureDef {
                density: 1.0,
                friction: 0.3,
                restitution: 0.5,
                ..b2::FixtureDef::new()
            },
        );
        FallingBox {
            body_handle: handle,
            color: color,
        }
    }

    fn extend_vertex_buffer(
        &self,
        world: &World,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
    ) {
        let start = vertices.len() as u32;
        let body = world.body(self.body_handle);
        let transform = body.transform();
        vertices.extend(&[
            Vertex {
                pos: *(transform * b2::Vec2 { x: 0.52, y: 0.52 }).as_array(),
                uv: [1.0, 0.0],
                color: self.color,
            },
            Vertex {
                pos: *(transform * b2::Vec2 { x: -0.52, y: 0.52 }).as_array(),
                uv: [0.0, 0.0],
                color: self.color,
            },
            Vertex {
                pos: *(transform * b2::Vec2 { x: -0.52, y: -0.52 }).as_array(),
                uv: [0.0, 1.0],
                color: self.color,
            },
            Vertex {
                pos: *(transform * b2::Vec2 { x: 0.52, y: -0.52 }).as_array(),
                uv: [1.0, 1.0],
                color: self.color,
            },
        ]);
        indices.extend(&[start, start + 1, start + 2, start + 2, start + 3, start]);
    }
}

struct App {
    world: World,
    boxes: Vec<FallingBox>,
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
            boxes: vec![],
            vertices: Vec::with_capacity(4 * 4096),
            indices: Vec::with_capacity(6 * 4096),
            pipeline: None,
            renderer: None,
        }
    }

    fn dump_data(&self, state: &PistonAppState) {
        let box_count = self.boxes.len();
        let vertex_count = box_count * 4;
        let index_count = box_count * 6;
        let memory = (box_count * std::mem::size_of::<FallingBox>()
            + vertex_count * std::mem::size_of::<Vertex>()
            + index_count * std::mem::size_of::<u32>()) as f32
            / 1024.0;
        println!(
            "Frame {} | Boxes: {} | Vertices: {} | Indices: {} | Memory: {:.2} KB",
            state.frame_count(),
            box_count,
            vertex_count,
            index_count,
            memory
        );
    }

    fn setup_world(&mut self, state: &PistonAppState) {
        let ground = self.world.create_body(&b2::BodyDef {
            position: b2::Vec2 { x: 0.0, y: -10.0 },
            ..b2::BodyDef::new()
        });
        let shape = b2::PolygonShape::new_box(
            state.width() as f32 * 4.2 / PIXELS_PER_METER,
            10.0,
        );
        self.world.body_mut(ground).create_fast_fixture(&shape, 0.0);
    }

    fn spawn_box(&mut self, state: &PistonAppState) {
        let x = (state.mouse_x() - state.width() / 2.0) as f32 / PIXELS_PER_METER;
        let y = (state.height() - state.mouse_y()) as f32 / PIXELS_PER_METER;
        let falling_box =
            FallingBox::new(&mut self.world, x, y, state.random_color(Some(1.0)));
        self.boxes.push(falling_box);
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.setup_world(state);
        let (pipeline, renderer) = PistonPipelineBuilder::new()
            .texture_atlas(TextureAtlas::from_path(window, "assets/box.png").unwrap())
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
            self.spawn_box(state);
        }
        self.vertices.clear();
        self.indices.clear();
        self.world.step(1.0 / 60.0, 8, 3);
        self.world.clear_forces();
        for falling_box in &self.boxes {
            falling_box.extend_vertex_buffer(
                &self.world,
                &mut self.vertices,
                &mut self.indices,
            );
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

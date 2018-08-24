//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Two-dimensional perlin noise.

#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate piston_app;

use gfx::traits::FactoryExt;
use piston_app::*;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "pos",
    }

    pipeline noise_quad {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        color: gfx::Global<[f32; 4]> = "color",
        time: gfx::Global<f32> = "time",
        out: gfx::RenderTarget<gfx::format::Srgba8> = "o_Color",
    }
}

struct NoiseQuad {
    slice: gfx::Slice<gfx_device_gl::Resources>,
    pipeline: gfx::pso::PipelineState<gfx_device_gl::Resources, noise_quad::Meta>,
    data: noise_quad::Data<gfx_device_gl::Resources>,
    color_offset: Scalar,
}

impl NoiseQuad {
    fn new(window: &mut PistonAppWindow) -> Self {
        const VERTICES: &[Vertex] = &[Vertex { pos: [1.0, -1.0] },
                                      Vertex { pos: [-1.0, -1.0] },
                                      Vertex { pos: [-1.0, 1.0] },
                                      Vertex { pos: [1.0, 1.0] }];
        const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];
        let factory = &mut window.factory;
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(VERTICES, INDICES);
        NoiseQuad {
            slice: slice,
            pipeline: factory
                .create_pipeline_simple(include_bytes!("noise_150_core.glslv"),
                                        include_bytes!("noise_150_core.glslf"),
                                        noise_quad::new())
                .unwrap(),
            data: noise_quad::Data {
                vbuf: vbuf,
                color: color::TRANSPARENT,
                time: 0.0,
                out: window.output_color.clone(),
            },
            color_offset: SmallRng::from_entropy().gen(),
        }
    }

    fn draw(&self, window: &mut PistonAppWindow) {
        let encoder = &mut window.encoder;
        encoder.draw(&self.slice, &self.pipeline, &self.data);
        encoder.flush(&mut window.device);
    }

    fn update(&mut self, state: &PistonAppState) {
        self.color_offset += 1e-3;
        self.data.color = state.noise_color(self.color_offset, Some(1.0));
        self.data.time += 0.00666;
    }
}

struct App {
    noise_quad: Option<NoiseQuad>,
}

impl App {
    fn new() -> Self {
        App { noise_quad: None }
    }

    fn noise_quad(&self) -> &NoiseQuad {
        self.noise_quad.as_ref().unwrap()
    }

    fn noise_quad_mut(&mut self) -> &mut NoiseQuad {
        self.noise_quad.as_mut().unwrap()
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, _: &PistonAppState) {
        self.noise_quad = Some(NoiseQuad::new(window));
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.noise_quad_mut().update(state);
        self.noise_quad().draw(window);
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

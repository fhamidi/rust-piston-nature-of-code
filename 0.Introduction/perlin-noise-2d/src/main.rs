//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Two-dimensional Perlin noise.

extern crate piston_app;

use piston_app::*;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "pos",
    }

    pipeline noise {
        vbuf: VertexBuffer<Vertex> = (),
        color: Global<[f32; 4]> = "color",
        time: Global<f32> = "time",
        out: RenderTarget<gfx::format::Srgba8> = "o_color",
    }
}

struct App {
    base_hue: Scalar,
    color_offset: Scalar,
    time: Scalar,
    pipeline: Option<PistonPipeline<noise::Meta>>,
    renderer: Option<PistonRenderer>,
}

impl App {
    fn new() -> Self {
        let mut rng = thread_rng();
        App {
            base_hue: rng.gen(),
            color_offset: rng.gen(),
            time: 0.0,
            pipeline: None,
            renderer: None,
        }
    }

    fn pipeline(&self) -> &PistonPipeline<noise::Meta> {
        self.pipeline.as_ref().unwrap()
    }

    fn renderer(&self) -> &PistonRenderer {
        self.renderer.as_ref().unwrap()
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, _: &PistonAppState) {
        let (pipeline, renderer) = PistonPipelineBuilder::new()
            .vertex_shader(include_bytes!("noise_150_core.glslv"))
            .fragment_shader(include_bytes!("noise_150_core.glslf"))
            .build(window, noise::new())
            .unwrap();
        self.pipeline = Some(pipeline);
        self.renderer = Some(renderer);
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        const VERTICES: &[Vertex] = &[Vertex { pos: [1.0, -1.0] },
                                      Vertex { pos: [-1.0, -1.0] },
                                      Vertex { pos: [-1.0, 1.0] },
                                      Vertex { pos: [1.0, 1.0] }];
        const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];
        self.color_offset += 1e-3;
        self.time += 0.00666;
        self.renderer()
            .draw(window, self.pipeline(), VERTICES, INDICES, |vbuf, out| {
                noise::Data {
                    vbuf: vbuf,
                    color: state.noise_color(self.base_hue, self.color_offset, Some(1.0)),
                    time: self.time as f32,
                    out: out,
                }
            });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

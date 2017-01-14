//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Two-dimensional perlin noise.

extern crate piston_app;

use piston_app::*;

type Canvas = img::ImageBuffer<img::Rgba<u8>, Vec<u8>>;

struct App {
    canvas: Option<Canvas>,
    texture: Option<G2dTexture>,
}

impl App {
    fn new() -> Self {
        App {
            canvas: None,
            texture: None,
        }
    }

    fn canvas(&self) -> &Canvas {
        self.canvas.as_ref().unwrap()
    }

    fn texture(&self) -> &G2dTexture {
        self.texture.as_ref().unwrap()
    }

    fn update_texture(&mut self, window: &mut PistonAppWindow) {
        self.texture
            .as_mut()
            .unwrap()
            .update(&mut window.encoder, self.canvas.as_ref().unwrap())
            .unwrap()
    }

    fn update(&mut self, state: &PistonAppState) {
        let ref mut canvas = self.canvas.as_mut().unwrap();
        let (width, height) = (state.width() as u32, state.height() as u32);
        let mut ty = 0.0;
        for y in 0..height {
            let mut tx = 0.0;
            for x in 0..width {
                let value =
                    state.map_range(state.noise(&[tx, ty]), 0.0, 1.0, 0.0, 256.0) as u8;
                canvas.put_pixel(x, y, img::Rgba([value, value, value, 255]));
                tx += 0.01;
            }
            ty += 0.01;
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.canvas = Some(img::ImageBuffer::new(state.width() as u32,
                                                 state.height() as u32));
        self.texture = Some(Texture::from_image(&mut window.factory,
                                                self.canvas(),
                                                &TextureSettings::new())
            .unwrap());
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.update(state);
        self.update_texture(window);
        window.draw_2d(state.event(), |context, gfx| {
            image(self.texture(), context.transform, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

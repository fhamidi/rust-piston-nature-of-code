//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Two-dimensional perlin noise.

extern crate piston_app;

use piston_app::*;

struct Background {
    canvas: TextureCanvas,
    color_offset: Scalar,
    time: Scalar,
}

impl Background {
    fn new(window: &mut PistonAppWindow, state: &PistonAppState) -> Self {
        Background {
            canvas: TextureCanvas::new(window, state.width(), state.height(), None),
            color_offset: 0.0,
            time: 0.0,
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        image(self.canvas.texture(), context.transform, gfx);
    }

    fn step(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        let (width, height) = (state.width() as u32, state.height() as u32);
        let color = state.noise_color(self.color_offset, Some(1.0));
        let time = self.time;
        self.canvas.update(window, |canvas| {
            let mut y_offset = 0.0;
            for y in 0..height {
                let mut x_offset = 0.0;
                for x in 0..width {
                    let value = state.noise(&[x_offset, y_offset,
                                 time]) as ColorComponent;
                    let rgba = img::Rgba([(color[0] * value * 256.0) as u8,
                                          (color[1] * value * 256.0) as u8,
                                          (color[2] * value * 256.0) as u8,
                                          255]);
                    canvas.put_pixel(x, y, rgba);
                    x_offset += 0.01;
                }
                y_offset += 0.01;
            }
        });
        self.color_offset += 1e-3;
        self.time += 0.01;
    }
}

struct App {
    background: Option<Background>,
}

impl App {
    fn new() -> Self {
        App { background: None }
    }

    fn background(&self) -> &Background {
        self.background.as_ref().unwrap()
    }

    fn background_mut(&mut self) -> &mut Background {
        self.background.as_mut().unwrap()
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.background = Some(Background::new(window, state));
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.background_mut().step(window, state);
        window.draw_2d(state.event(), |context, gfx| {
            self.background().draw(context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

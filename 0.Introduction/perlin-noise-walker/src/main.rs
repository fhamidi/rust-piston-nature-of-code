//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Perlin noise walker.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Walker {
    color: Color,
    x: Scalar,
    y: Scalar,
    color_offset: Scalar,
    x_offset: Scalar,
    y_offset: Scalar,
}

impl Walker {
    fn new() -> Self {
        Walker {
            color: color::TRANSPARENT,
            x: 0.0,
            y: 0.0,
            color_offset: 0.0,
            x_offset: 1e3,
            y_offset: 1e6,
        }
    }

    fn set_position(&mut self, x: Scalar, y: Scalar) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .resolution(32)
            .color(self.color)
            .draw(ellipse::circle(self.x, self.y, 32.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn update(&mut self, state: &PistonAppState) {
        self.color = state.noise_color(self.color_offset, Some(1.0));
        self.x = state.map_x(state.noise(&[self.x_offset]));
        self.y = state.map_y(state.noise(&[self.y_offset]));
        self.color_offset += 1e-3;
        self.x_offset += 0.01;
        self.y_offset += 0.01;
    }
}

#[derive(Debug)]
struct App {
    walker: Walker,
}

impl App {
    fn new() -> Self {
        App { walker: Walker::new() }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        self.walker
            .set_position(state.width() / 2.0, state.height() / 2.0);
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.walker.update(state);
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            self.walker.draw(context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

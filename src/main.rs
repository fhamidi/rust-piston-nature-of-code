//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Perlin noise walker.

extern crate piston_app;
extern crate rand;

use piston_app::*;

#[derive(Debug)]
struct Walker {
    color: Color,
    x: Scalar,
    y: Scalar,
    tcolor: Scalar,
    tx: Scalar,
    ty: Scalar,
}

impl Walker {
    fn new() -> Self {
        Walker {
            color: color::TRANSPARENT,
            x: 0.0,
            y: 0.0,
            tcolor: 0.0,
            tx: 1e3,
            ty: 1e6,
        }
    }

    fn set_position(&mut self, x: Scalar, y: Scalar) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self, context: Context, gl: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .color(self.color)
            .draw(ellipse::circle(self.x, self.y, 32.0),
                  &context.draw_state,
                  context.transform,
                  gl);
    }

    fn step(&mut self, state: &PistonAppState) {
        self.color = state.noise_color(self.tcolor, Some(1.0));
        self.x = state.map_x(state.noise(&[self.tx]));
        self.y = state.map_y(state.noise(&[self.ty]));
        self.tcolor += 1e-3;
        self.tx += 0.01;
        self.ty += 0.01;
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
    fn setup(&mut self, _: Context, _: &mut G2d, state: &PistonAppState) {
        self.walker.set_position(state.width() / 2.0, state.height() / 2.0);
    }

    fn draw(&mut self, context: Context, gl: &mut G2d, state: &PistonAppState) {
        self.walker.step(state);
        clear(color::WHITE, gl);
        self.walker.draw(context, gl);
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

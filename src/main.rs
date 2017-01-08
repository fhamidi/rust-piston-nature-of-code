//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Random walker.

extern crate piston_app;
extern crate rand;

use piston_app::*;
use piston_app::math::*;
use rand::Rng;

#[derive(Debug)]
struct Walker {
    x: Scalar,
    y: Scalar,
}

impl Walker {
    fn new() -> Self {
        Walker { x: 0.0, y: 0.0 }
    }

    fn set_position(&mut self, x: Scalar, y: Scalar) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self, context: Context, gl: &mut G2d) {
        rectangle([0.0, 0.0, 0.0, 1.0],
                  rectangle::square(self.x, self.y, 1.0),
                  context.transform,
                  gl);
    }

    fn step(&mut self) {
        let mut rng = rand::thread_rng();
        let step_x = rng.gen_range(-1.0, 1.0);
        let step_y = rng.gen_range(-1.0, 1.0);
        self.x += step_x;
        self.y += step_y;
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
    fn setup(&mut self, _: Context, gl: &mut G2d, args: &RenderArgs) {
        self.walker.set_position(args.width as Scalar / 2.0, args.height as Scalar / 2.0);
        clear([1.0; 4], gl);
    }

    fn draw(&mut self, context: Context, gl: &mut G2d, _: &RenderArgs) {
        self.walker.step();
        self.walker.draw(context, gl);
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

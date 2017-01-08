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
    x: i32,
    y: i32,
}

impl Walker {
    fn new() -> Self {
        Walker { x: 0, y: 0 }
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self, context: Context, gl: &mut G2d) {
        rectangle([0.0, 0.0, 0.0, 1.0],
                  rectangle::square(self.x as Scalar, self.y as Scalar, 1.0),
                  context.transform,
                  gl);
    }

    fn step(&mut self) {
        let choice = rand::thread_rng().gen_range(0, 4);
        match choice {
            0 => self.x += 1,
            1 => self.x -= 1,
            2 => self.y += 1,
            3 => self.y -= 1,
            _ => unreachable!()
        }
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
        self.walker.set_position(args.width as i32 / 2, args.height as i32 / 2);
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

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Random distribution graph.

extern crate piston_app;
extern crate rand;

use piston_app::*;

#[derive(Debug)]
struct App {
    random_counts: [u32; 20],
    colors: [Color; 20],
}

impl App {
    fn new() -> Self {
        let mut app = App {
            random_counts: [0; 20],
            colors: [[0.0; 4]; 20],
        };
        for i in 0..app.colors.len() {
            app.colors[i] = random_color();
        }
        app
    }
}

impl PistonApp for App {
    fn draw(&mut self, context: Context, gl: &mut G2d, args: &RenderArgs) {
        let length = self.random_counts.len();
        let index = rand::thread_rng().gen_range(0, length);
        self.random_counts[index] += 1;
        let width = args.width as Scalar / length as Scalar;
        let height = args.height as Scalar;
        clear([1.0; 4], gl);
        for x in 0..length {
            let count = self.random_counts[x] as Scalar;
            Rectangle::new_border([0.0, 0.0, 0.0, 1.0], 1.0)
                .color(self.colors[x])
                .draw([x as Scalar * width, height - count, width - 1.0, count],
                      &context.draw_state,
                      context.transform,
                      gl);
        }
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

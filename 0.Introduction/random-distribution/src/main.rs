//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Random distribution graph.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct App {
    random_counts: [u32; 20],
    colors: [Color; 20],
}

impl App {
    fn new() -> Self {
        App {
            random_counts: [0; 20],
            colors: [color::TRANSPARENT; 20],
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        for i in 0..self.colors.len() {
            self.colors[i] = state.random_color(Some(1.0));
        }
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        let length = self.random_counts.len();
        let index = SmallRng::from_entropy().gen_range(0, length);
        self.random_counts[index] += 1;
        let width = state.width() / length as Scalar;
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            for x in 0..length {
                let count = self.random_counts[x] as Scalar;
                let coords =
                    [x as Scalar * width, state.height() - count, width - 1.0, count];
                Rectangle::new_border(color::BLACK, 1.0)
                    .color(self.colors[x])
                    .draw(coords, &context.draw_state, context.transform, gfx);
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Gaussian distribution simulation.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct App;

impl App {
    fn new() -> Self {
        App {}
    }
}

impl PistonApp for App {
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        window.draw_2d(state.event(), |_, gfx| clear(color::WHITE, gfx));
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        let sd = 66.6;
        let mean = state.width() / 2.0;
        let x = SmallRng::from_entropy().sample(StandardNormal) * sd + mean;
        window.draw_2d(state.event(), |context, gfx| {
            Ellipse::new([0.0, 0.0, 0.0, 0.1])
                .resolution(32)
                .draw(ellipse::circle(x, state.height() / 2.0, 16.0),
                      &context.draw_state,
                      context.transform,
                      gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

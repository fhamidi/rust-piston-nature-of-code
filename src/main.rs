//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Random walker.
//!

extern crate piston_app;

use piston_app::*;

struct App;

impl App {
    pub fn new() -> Self {
        App {}
    }
}

impl PistonApp for App {
    fn setup<W, E>(&mut self, window: &mut PistonWindow<W>, e: &E, args: &RenderArgs)
        where E: GenericEvent,
              W: OpenGLWindow,
              W::Event: GenericEvent
    {
        window.draw_2d(e, |_, gl| {
            clear([1.0, 1.0, 1.0, 1.0], gl);
        });
    }

    fn draw<W, E>(&mut self, window: &mut PistonWindow<W>, e: &E, args: &RenderArgs)
        where E: GenericEvent,
              W: OpenGLWindow,
              W::Event: GenericEvent
    {
        window.draw_2d(e, |context, gl| {
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

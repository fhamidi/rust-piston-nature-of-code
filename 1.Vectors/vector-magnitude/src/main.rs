//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Vectors - Magnitude of difference between mouse pointer and window center.

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
    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        let center = [state.width() / 2.0, state.height() / 2.0];
        let mouse = vec2_sub([state.mouse_x(), state.mouse_y()], center);
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            rectangle(
                color::BLACK,
                [0.0, 0.0, vec2_len(mouse), 16.0],
                context.transform,
                gfx,
            );
            let line = Line::new_round(color::BLACK, 3.0);
            line.draw(
                [0.0, 0.0, mouse[0], mouse[1]],
                &context.draw_state,
                context.transform.trans(center[0], center[1]),
                gfx,
            );
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

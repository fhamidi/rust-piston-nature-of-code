//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Random walker.

extern crate piston_app;

use piston_app::*;

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

    fn draw(&self, context: Context, gfx: &mut G2d) {
        rectangle(color::BLACK,
                  rectangle::square(self.x, self.y, 1.0),
                  context.transform,
                  gfx);
    }

    fn update(&mut self, state: &PistonAppState) {
        let mut rng = SmallRng::from_entropy();
        if state.mouse_pressed() && rng.gen() {
            self.x += rng.gen::<Scalar>() * (state.mouse_x() - self.x).signum();
            self.y += rng.gen::<Scalar>() * (state.mouse_y() - self.y).signum();
        } else {
            self.x += rng.gen_range(-1.0, 1.0);
            self.y += rng.gen_range(-1.0, 1.0);
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
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.walker
            .set_position(state.width() / 2.0, state.height() / 2.0);
        window.draw_2d(state.event(), |_, gfx| clear(color::WHITE, gfx));
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.walker.update(state);
        window.draw_2d(state.event(), |context, gfx| self.walker.draw(context, gfx));
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

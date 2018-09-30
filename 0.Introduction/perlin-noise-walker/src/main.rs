//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Introduction - Perlin noise walker.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Walker {
    base_hue: Scalar,
    color_offset: Scalar,
    x_offset: Scalar,
    y_offset: Scalar,
}

impl Walker {
    fn new() -> Self {
        let mut rng = thread_rng();
        Walker {
            base_hue: rng.gen(),
            color_offset: rng.gen(),
            x_offset: 1e3,
            y_offset: 1e6,
        }
    }

    fn draw(&self, state: &PistonAppState, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .resolution(32)
            .color(state.noise_color(self.base_hue, self.color_offset, Some(1.0)))
            .draw(
                ellipse::circle(
                    state.map_x(state.noise(&[self.x_offset])),
                    state.map_y(state.noise(&[self.y_offset])),
                    32.0,
                ),
                &context.draw_state,
                context.transform,
                gfx,
            );
    }

    fn update(&mut self) {
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
        App {
            walker: Walker::new(),
        }
    }
}

impl PistonApp for App {
    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.walker.update();
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            self.walker.draw(state, context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

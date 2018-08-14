//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Angular motion through rotating baton.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct App {
    angle: Scalar,
    angular_velocity: Scalar,
    angular_acceleration: Scalar,
    baton_colors: [Color; 2],
}

impl App {
    fn new() -> Self {
        App {
            angle: 0.0,
            angular_velocity: 0.0,
            angular_acceleration: 1e-4,
            baton_colors: [color::TRANSPARENT, color::TRANSPARENT],
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        self.baton_colors[0] = state.random_color(Some(1.0));
        self.baton_colors[1] = state.random_color(Some(1.0));
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            let transform = context
                .transform
                .trans(state.width() / 2.0, state.height() / 2.0)
                .rot_rad(self.angle);
            line(color::BLACK, 2.0, [-96.0, 0.0, 96.0, 0.0], transform, gfx);
            for i in 0..self.baton_colors.len() {
                let axis = match i {
                    0 => -1.0,
                    _ => 1.0,
                };
                Ellipse::new_border(color::BLACK, 1.0)
                    .resolution(16)
                    .color(self.baton_colors[i])
                    .draw(ellipse::circle(108.0 * axis, 0.0, 12.0),
                          &context.draw_state,
                          transform,
                          gfx);
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

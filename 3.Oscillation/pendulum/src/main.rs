//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Pendulum.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Pendulum {
    color: Color,
    origin: Vec2d,
    arm_length: Scalar,
    angle: Scalar,
    angular_velocity: Scalar,
    angular_acceleration: Scalar,
    damping: Scalar,
}

impl Pendulum {
    fn new() -> Self {
        Pendulum {
            color: color::TRANSPARENT,
            origin: [0.0, 0.0],
            arm_length: 0.0,
            angle: consts::FRAC_PI_4,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            damping: 0.995,
        }
    }

    fn setup(&mut self, state: &PistonAppState) {
        self.color = state.random_color(Some(1.0));
        self.origin = [state.width() / 2.0, 0.0];
        self.arm_length = state.height() * 0.8;
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        let x = self.angle.sin() * self.arm_length + self.origin[0];
        let y = self.angle.cos() * self.arm_length + self.origin[1];
        Line::new(color::BLACK, 1.0).draw([self.origin[0], self.origin[1], x, y],
                                          &context.draw_state,
                                          context.transform,
                                          gfx);
        Ellipse::new_border(color::BLACK, 1.0)
            .color(self.color)
            .draw(ellipse::circle(x, y, 24.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn update(&mut self) {
        const GRAVITY: Scalar = 0.42;
        self.angular_acceleration = -GRAVITY / self.arm_length * self.angle.sin();
        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;
        self.angular_velocity *= self.damping;
    }
}

#[derive(Debug)]
struct App {
    pendulum: Pendulum,
}

impl App {
    fn new() -> Self {
        App { pendulum: Pendulum::new() }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        self.pendulum.setup(state);
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.pendulum.update();
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            self.pendulum.draw(context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

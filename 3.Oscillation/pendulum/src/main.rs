//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Pendulum.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Pendulum {
    anchor_color: Color,
    anchor_position: Vec2d,
    bob_color: Color,
    bob_position: Vec2d,
    bob_radius: Scalar,
    length: Scalar,
    angle: Scalar,
    angular_velocity: Scalar,
    angular_acceleration: Scalar,
    damping: Scalar,
    dragging: bool,
}

impl Pendulum {
    fn new() -> Self {
        Pendulum {
            anchor_color: color::TRANSPARENT,
            anchor_position: [0.0, 0.0],
            bob_color: color::TRANSPARENT,
            bob_position: [0.0, 0.0],
            bob_radius: 32.0,
            length: 0.0,
            angle: consts::FRAC_PI_4,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            damping: 0.996,
            dragging: false,
        }
    }

    fn setup(&mut self, state: &PistonAppState) {
        self.anchor_color = state.random_color(Some(1.0));
        self.anchor_position = [state.width() / 2.0, state.height() / 24.0];
        self.bob_color = state.random_color(Some(1.0));
        self.length = state.height() * 0.84;
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        let (anchor_x, anchor_y) = (self.anchor_position[0], self.anchor_position[1]);
        let (bob_x, bob_y) = (self.bob_position[0], self.bob_position[1]);
        Line::new(color::BLACK, 1.0).draw([anchor_x, anchor_y, bob_x, bob_y],
                                          &context.draw_state,
                                          context.transform,
                                          gfx);
        Rectangle::new_border(color::BLACK, 1.0)
            .color(self.anchor_color)
            .draw(rectangle::centered_square(anchor_x, anchor_y, 8.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
        Ellipse::new_border(color::BLACK, 1.0)
            .resolution(32)
            .color(if self.dragging {
                       color::BLACK
                   } else {
                       self.bob_color
                   })
            .draw(ellipse::circle(bob_x, bob_y, self.bob_radius),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn update(&mut self, state: &PistonAppState) {
        const GRAVITY: Scalar = 0.42;
        let button_pressed = state.mouse_button_pressed(MouseButton::Left);
        if self.dragging {
            if !button_pressed {
                self.angular_velocity = 0.0;
                self.dragging = false;
            }
        } else if button_pressed {
            let distance = vec2_len(vec2_sub([state.mouse_x(), state.mouse_y()],
                                             self.bob_position));
            if distance < self.bob_radius {
                self.dragging = true;
            }
        }
        if self.dragging {
            let direction = vec2_sub([state.mouse_x(), state.mouse_y()],
                                     self.anchor_position);
            self.angle = -vec2_heading(direction) + consts::FRAC_PI_2;
        } else {
            self.angular_acceleration = -GRAVITY / self.length * self.angle.sin();
            self.angular_velocity += self.angular_acceleration;
            self.angle += self.angular_velocity;
            self.angular_velocity *= self.damping;
        }
        self.bob_position = [self.angle.sin() * self.length + self.anchor_position[0],
                             self.angle.cos() * self.length + self.anchor_position[1]];
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
        self.pendulum.update(state);
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

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Simulation of a spaceship, driven by the arrow keys.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Spaceship {
    color: Color,
    radius: Scalar,
    location: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
    heading: Scalar,
    damping: Scalar,
    top_speed: Scalar,
    thrusting: bool,
}

impl Spaceship {
    fn new() -> Self {
        Spaceship {
            color: color::TRANSPARENT,
            radius: 16.0,
            location: [0.0, 0.0],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
            heading: 0.0,
            damping: 0.995,
            top_speed: 6.0,
            thrusting: false,
        }
    }

    fn setup(&mut self, state: &PistonAppState) {
        self.color = state.random_color(Some(1.0));
        self.location = [state.width() / 2.0, state.height() / 2.0];
    }

    fn draw(&mut self, context: Context, gfx: &mut G2d) {
        let transform = context.transform
            .trans(self.location[0], self.location[1] + self.radius)
            .rot_rad(self.heading);
        let thruster_color = if self.thrusting {
            [1.0, 0.0, 0.0, 1.0]
        } else {
            [0.75, 0.75, 0.75, 1.0]
        };
        let thruster = Rectangle::new_border(color::BLACK, 1.0).color(thruster_color);
        thruster.draw(rectangle::centered([-self.radius / 2.0,
                                           self.radius,
                                           self.radius / 6.0,
                                           self.radius / 4.0]),
                      &context.draw_state,
                      transform,
                      gfx);
        thruster.draw(rectangle::centered([self.radius / 2.0,
                                           self.radius,
                                           self.radius / 6.0,
                                           self.radius / 4.0]),
                      &context.draw_state,
                      transform,
                      gfx);
        let vertices = [[-self.radius, self.radius],
                        [0.0, -self.radius],
                        [self.radius, self.radius]];
        let border = Line::new_round(color::BLACK, 1.0);
        polygon(self.color, &vertices, transform, gfx);
        border.draw([vertices[0][0], vertices[0][1], vertices[1][0], vertices[1][1]],
                    &context.draw_state,
                    transform,
                    gfx);
        border.draw([vertices[1][0], vertices[1][1], vertices[2][0], vertices[2][1]],
                    &context.draw_state,
                    transform,
                    gfx);
        border.draw([vertices[2][0], vertices[2][1], vertices[0][0], vertices[0][1]],
                    &context.draw_state,
                    transform,
                    gfx);
        self.thrusting = false;
    }

    fn update(&mut self, state: &PistonAppState) {
        let velocity = vec2_scale(vec2_add(self.velocity, self.acceleration),
                                  self.damping);
        self.velocity = vec2_limit(velocity, self.top_speed);
        self.location = vec2_add(self.location, self.velocity);
        self.acceleration = [0.0, 0.0];
        self.check_edges(state);
    }

    fn check_edges(&mut self, state: &PistonAppState) {
        let buffer = self.radius * 2.0;
        let (x, y) = (self.location[0], self.location[1]);
        let (width, height) = (state.width(), state.height());
        if x > width + buffer {
            self.location[0] = -buffer;
        } else if x < -buffer {
            self.location[0] = width + buffer;
        }
        if y > height + buffer {
            self.location[1] = -buffer;
        } else if y < -buffer {
            self.location[1] = height + buffer;
        }
    }

    fn apply_force(&mut self, force: Vec2d) {
        self.acceleration = vec2_add(self.acceleration, force);
    }

    fn turn(&mut self, angle: Scalar) {
        self.heading += angle;
    }

    fn thrust(&mut self) {
        let angle = self.heading - consts::FRAC_PI_2;
        let force = vec2_scale([angle.cos(), angle.sin()], 0.1);
        self.apply_force(force);
        self.thrusting = true;
    }
}

#[derive(Debug)]
struct App {
    ship: Spaceship,
}

impl App {
    fn new() -> Self {
        App { ship: Spaceship::new() }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        self.ship.setup(state);
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        if state.key_pressed() {
            match state.key() {
                Key::Left => self.ship.turn(-0.03),
                Key::Right => self.ship.turn(0.03),
                Key::Up => self.ship.thrust(),
                _ => (),
            }
        }
        self.ship.update(state);
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            self.ship.draw(context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Spring forces.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Bob {
    color: Color,
    position: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
    mass: Scalar,
    radius: Scalar,
    damping: Scalar,
    dragging: bool,
}

impl Bob {
    fn new() -> Self {
        Bob {
            color: color::TRANSPARENT,
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
            mass: 24.0,
            radius: 32.0,
            damping: 0.996,
            dragging: false,
        }
    }

    #[inline]
    fn position(&self) -> Vec2d {
        self.position
    }

    fn setup(&mut self, state: &PistonAppState) {
        self.color = state.random_color(Some(1.0));
        self.position = [state.width() * 0.8, state.height() * 0.666];
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .color(if self.dragging {
                color::BLACK
            } else {
                self.color
            }).draw(
                ellipse::circle(self.position[0], self.position[1], self.radius),
                &context.draw_state,
                context.transform,
                gfx,
            );
    }

    fn apply_force(&mut self, force: Vec2d) {
        self.acceleration =
            vec2_add(self.acceleration, vec2_scale(force, 1.0 / self.mass));
    }

    fn update(&mut self, state: &PistonAppState) {
        let button_pressed = state.mouse_button_pressed(MouseButton::Left);
        if self.dragging {
            if !button_pressed {
                self.velocity = [0.0, 0.0];
                self.dragging = false;
            }
        } else if button_pressed {
            let distance =
                vec2_len(vec2_sub([state.mouse_x(), state.mouse_y()], self.position));
            if distance < self.radius {
                self.dragging = true;
            }
        }
        if self.dragging {
            self.position = [state.mouse_x(), state.mouse_y()];
        } else {
            self.velocity =
                vec2_scale(vec2_add(self.velocity, self.acceleration), self.damping);
            self.position = vec2_add(self.position, self.velocity);
        }
        self.acceleration = [0.0, 0.0];
    }
}

#[derive(Debug)]
struct Spring {
    anchor_color: Color,
    anchor_position: Vec2d,
    length: Scalar,
    k: Scalar,
}

impl Spring {
    fn new() -> Self {
        Spring {
            anchor_color: color::TRANSPARENT,
            anchor_position: [0.0, 0.0],
            length: 0.0,
            k: 0.24,
        }
    }

    fn setup(&mut self, state: &PistonAppState) {
        self.anchor_color = state.random_color(Some(1.0));
        self.anchor_position = [state.width() / 2.0, state.height() / 24.0];
        self.length = state.height() * 0.42;
    }

    fn draw(&self, bob: &Bob, context: Context, gfx: &mut G2d) {
        let (anchor_x, anchor_y) = (self.anchor_position[0], self.anchor_position[1]);
        let bob_position = bob.position();
        Line::new(color::BLACK, 1.0).draw(
            [anchor_x, anchor_y, bob_position[0], bob_position[1]],
            &context.draw_state,
            context.transform,
            gfx,
        );
        Rectangle::new_border(color::BLACK, 1.0)
            .color(self.anchor_color)
            .draw(
                rectangle::centered_square(anchor_x, anchor_y, 8.0),
                &context.draw_state,
                context.transform,
                gfx,
            );
    }

    fn connect(&self, bob: &mut Bob) {
        let force = vec2_sub(bob.position(), self.anchor_position);
        let delta = vec2_len(force) - self.length;
        bob.apply_force(vec2_scale(vec2_normalized(force), -self.k * delta));
    }
}

#[derive(Debug)]
struct App {
    bob: Bob,
    spring: Spring,
}

impl App {
    fn new() -> Self {
        App {
            bob: Bob::new(),
            spring: Spring::new(),
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        self.bob.setup(state);
        self.spring.setup(state);
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        const GRAVITY: Vec2d = [0.0, 0.42];
        self.bob.apply_force(GRAVITY);
        self.spring.connect(&mut self.bob);
        self.bob.update(state);
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            self.spring.draw(&self.bob, context, gfx);
            self.bob.draw(context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Vectors - Keyboard-controlled acceleration.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Mover {
    color: Color,
    location: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
}

impl Mover {
    fn new(state: &PistonAppState) -> Self {
        Mover {
            color: color::TRANSPARENT,
            location: [state.width() / 2.0, state.height() / 2.0],
            velocity: [1.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .resolution(32)
            .color(self.color)
            .draw(ellipse::circle(self.location[0], self.location[1], 32.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn update(&mut self, state: &PistonAppState) {
        const MAX_VELOCITY: Scalar = 9.0;
        if state.key_pressed() {
            match state.key() {
                Key::Down | Key::Left => self.acceleration[0] -= 6e-3,
                Key::Up | Key::Right => self.acceleration[0] += 1e-3,
                _ => (),
            }
        }
        self.velocity = vec2_limit(vec2_add(self.velocity, self.acceleration),
                                   MAX_VELOCITY);
        if self.velocity[0] < 0.0 {
            self.acceleration[0] = 0.0;
            self.velocity[0] = 0.0;
        }
        self.location = vec2_add(self.location, self.velocity);
        self.check_edges(state);
        let hue = state.map_range(vec2_len(self.velocity), 0.0, MAX_VELOCITY, 0.0, 120.0);
        self.color = state.color_from_hsv(hue, 1.0, 2.0 / 3.0, 1.0);
    }

    fn check_edges(&mut self, state: &PistonAppState) {
        let (x, y) = (self.location[0], self.location[1]);
        let (width, height) = (state.width(), state.height());
        if x > width {
            self.location[0] = 0.0;
        } else if x < 0.0 {
            self.location[0] = width;
        }
        if y > height {
            self.location[1] = 0.0;
        } else if y < 0.0 {
            self.location[1] = height;
        }
    }
}

#[derive(Debug)]
struct App {
    mover: Option<Mover>,
}

impl App {
    fn new() -> Self {
        App { mover: None }
    }

    fn mover(&self) -> &Mover {
        self.mover.as_ref().unwrap()
    }

    fn mover_mut(&mut self) -> &mut Mover {
        self.mover.as_mut().unwrap()
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        self.mover = Some(Mover::new(state));
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.mover_mut().update(state);
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            self.mover().draw(context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

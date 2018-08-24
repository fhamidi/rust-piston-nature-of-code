//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Vectors - Motion 101 (random acceleration).

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Mover {
    color: Color,
    position: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
}

impl Mover {
    fn new(state: &PistonAppState) -> Self {
        Mover {
            color: color::TRANSPARENT,
            position: [state.width() / 2.0, state.height() / 2.0],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .color(self.color)
            .draw(ellipse::circle(self.position[0], self.position[1], 32.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn update(&mut self, state: &PistonAppState) {
        const MAX_VELOCITY: Scalar = 9.0;
        const MAX_ACCELERATION: Scalar = 2.0;
        self.acceleration = vec2_scale(vec2_random(),
                                       SmallRng::from_entropy()
                                           .gen_range(0.0, MAX_ACCELERATION));
        self.velocity = vec2_limit(vec2_add(self.velocity, self.acceleration),
                                   MAX_VELOCITY);
        self.position = vec2_add(self.position, self.velocity);
        self.check_edges(state);
        let hue = state.map_range(vec2_len(self.velocity), 0.0, MAX_VELOCITY, 0.0, 120.0);
        self.color = state.color_from_hsv(hue, 1.0, 2.0 / 3.0, 1.0);
    }

    fn check_edges(&mut self, state: &PistonAppState) {
        let (x, y) = (self.position[0], self.position[1]);
        let (width, height) = (state.width(), state.height());
        if x > width {
            self.position[0] = 0.0;
        } else if x < 0.0 {
            self.position[0] = width;
        }
        if y > height {
            self.position[1] = 0.0;
        } else if y < 0.0 {
            self.position[1] = height;
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

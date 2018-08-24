//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Vectors - Motion 101 (velocity).

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Mover {
    color: Color,
    position: Vec2d,
    velocity: Vec2d,
}

impl Mover {
    fn new(state: &PistonAppState) -> Self {
        const MAX_VELOCITY: Scalar = 6.0;
        let mut rng = SmallRng::from_entropy();
        Mover {
            color: state.random_color(Some(1.0)),
            position: [rng.gen_range(0.0, state.width()),
                       rng.gen_range(0.0, state.height())],
            velocity: [rng.gen_range(-MAX_VELOCITY, MAX_VELOCITY),
                       rng.gen_range(-MAX_VELOCITY, MAX_VELOCITY)],
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .resolution(32)
            .color(self.color)
            .draw(ellipse::circle(self.position[0], self.position[1], 32.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn update(&mut self, state: &PistonAppState) {
        self.position = vec2_add(self.position, self.velocity);
        self.check_edges(state);
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

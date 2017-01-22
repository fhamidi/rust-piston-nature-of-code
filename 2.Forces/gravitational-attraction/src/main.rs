//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Forces - Gravitational attraction.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Attractor {
    color: Color,
    location: Vec2d,
    mass: Scalar,
    g: Scalar,
}

impl Attractor {
    fn new(x: Scalar, y: Scalar, mass: Scalar, g: Scalar, color: Color) -> Self {
        Attractor {
            color: color,
            location: [x, y],
            mass: mass,
            g: g,
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 3.0)
            .color(self.color)
            .draw(ellipse::circle(self.location[0], self.location[1], self.mass * 2.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn attract(&self, mover: &Mover) -> Vec2d {
        let force = vec2_sub(self.location, mover.location());
        let distance = vec2_len(force).max(5.0).min(25.0);
        vec2_scale(vec2_normalized(force),
                   (self.g * self.mass * mover.mass()) / (distance * distance))
    }
}

#[derive(Debug)]
struct Mover {
    color: Color,
    location: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
    mass: Scalar,
}

impl Mover {
    fn new(x: Scalar, y: Scalar, mass: Scalar, color: Color) -> Self {
        Mover {
            color: color,
            location: [x, y],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
            mass: mass,
        }
    }

    #[inline]
    fn location(&self) -> Vec2d {
        self.location
    }

    #[inline]
    fn mass(&self) -> Scalar {
        self.mass
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .color(self.color)
            .draw(ellipse::circle(self.location[0], self.location[1], self.mass * 8.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn apply_force(&mut self, force: Vec2d) {
        self.acceleration = vec2_add(self.acceleration,
                                     vec2_scale(force, 1.0 / self.mass));
    }

    fn update(&mut self) {
        self.velocity = vec2_add(self.velocity, self.acceleration);
        self.location = vec2_add(self.location, self.velocity);
        self.acceleration = [0.0, 0.0];
    }
}

#[derive(Debug)]
struct App {
    attractors: Vec<Attractor>,
    movers: Vec<Mover>,
}

impl App {
    fn new() -> Self {
        App {
            attractors: vec![],
            movers: vec![],
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_G: Scalar = 0.8;
        const MAX_MOVERS: usize = 16;
        let mut rng = rand::thread_rng();
        let (width, height) = (state.width(), state.height());
        self.attractors.push(Attractor::new(width / 2.0,
                                            height / 2.0,
                                            20.0,
                                            MAX_G / 2.0,
                                            state.random_color(Some(1.0))));
        self.movers = (0..MAX_MOVERS)
            .map(|_| {
                Mover::new(rng.gen_range(0.0, width),
                           rng.gen_range(0.0, height),
                           rng.gen_range(0.1, 4.2),
                           state.random_color(Some(2.0 / 3.0)))
            })
            .collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        for mover in &mut self.movers {
            for attractor in &self.attractors {
                let force = attractor.attract(mover);
                mover.apply_force(force);
            }
            mover.update();
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            for attractor in &self.attractors {
                attractor.draw(context, gfx);
            }
            for mover in &self.movers {
                mover.draw(context, gfx);
            }
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

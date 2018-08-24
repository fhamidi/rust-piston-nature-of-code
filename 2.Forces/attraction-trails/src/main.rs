//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Forces - Render trails produced by gravitational attraction.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Attractor {
    position: Vec2d,
    mass: Scalar,
    g: Scalar,
}

impl Attractor {
    fn new(x: Scalar, y: Scalar, mass: Scalar, g: Scalar) -> Self {
        Attractor {
            position: [x, y],
            mass: mass,
            g: g,
        }
    }

    fn attract(&self, mover: &Mover) -> Vec2d {
        let force = vec2_sub(self.position, mover.position());
        let distance = vec2_len(force).max(5.0).min(25.0);
        vec2_scale(vec2_normalized(force),
                   (self.g * self.mass * mover.mass()) / (distance * distance))
    }
}

#[derive(Debug)]
struct Mover {
    color: Color,
    position: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
    mass: Scalar,
}

impl Mover {
    fn new(color: Color, x: Scalar, y: Scalar, mass: Scalar) -> Self {
        Mover {
            color: color,
            position: [x, y],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
            mass: mass,
        }
    }

    #[inline]
    fn position(&self) -> Vec2d {
        self.position
    }

    #[inline]
    fn mass(&self) -> Scalar {
        self.mass
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        rectangle(self.color,
                  rectangle::centered_square(self.position[0],
                                             self.position[1],
                                             self.mass / 4.2),
                  context.transform,
                  gfx);
    }

    fn apply_force(&mut self, force: Vec2d) {
        self.acceleration = vec2_add(self.acceleration,
                                     vec2_scale(force, 1.0 / self.mass));
    }

    fn update(&mut self) {
        self.velocity = vec2_add(self.velocity, self.acceleration);
        self.position = vec2_add(self.position, self.velocity);
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
    fn setup(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_G: Scalar = 0.8;
        const MAX_ATTRACTORS: usize = 4;
        const MAX_MOVERS: usize = 32;
        let mut rng = SmallRng::from_entropy();
        let (width, height) = (state.width(), state.height());
        self.attractors = (0..MAX_ATTRACTORS)
            .map(|_| {
                     Attractor::new(rng.gen_range(0.0, width),
                                    rng.gen_range(0.0, height),
                                    rng.gen_range(8.0, 32.0),
                                    rng.gen_range(MAX_G / 4.2, MAX_G))
                 })
            .collect();
        self.movers = (0..MAX_MOVERS)
            .map(|_| {
                     Mover::new(state.random_color(None),
                                rng.gen_range(0.0, width),
                                rng.gen_range(0.0, height),
                                rng.gen_range(0.1, 4.2))
                 })
            .collect();
        window.draw_2d(state.event(), |_, gfx| { clear(color::WHITE, gfx); });
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        for mover in &mut self.movers {
            for attractor in &self.attractors {
                let force = attractor.attract(mover);
                mover.apply_force(force);
            }
            mover.update();
        }
        window.draw_2d(state.event(), |context, gfx| for mover in &self.movers {
            mover.draw(context, gfx);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

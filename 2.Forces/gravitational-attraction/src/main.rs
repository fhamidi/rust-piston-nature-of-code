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
    fn new(color: Color, x: Scalar, y: Scalar, mass: Scalar, g: Scalar) -> Self {
        Attractor {
            color: color,
            location: [x, y],
            mass: mass,
            g: g,
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d, alpha: ColorComponent) {
        Ellipse::new_border([0.0, 0.0, 0.0, alpha], 2.0 + 4.0 * self.g)
            .resolution(self.mass as Resolution)
            .color([self.color[0], self.color[1], self.color[2], alpha])
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
    fn new(color: Color, x: Scalar, y: Scalar, mass: Scalar) -> Self {
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
            .resolution(self.mass as Resolution * 12)
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
    attractors_alpha: ColorComponent,
    attractors: Vec<Attractor>,
    movers: Vec<Mover>,
}

impl App {
    fn new() -> Self {
        App {
            attractors_alpha: 0.0,
            attractors: vec![],
            movers: vec![],
        }
    }

    fn handle_mouse(&mut self, state: &PistonAppState) {
        let mut delta = -0.1;
        if state.mouse_pressed() {
            delta = -delta;
        }
        self.attractors_alpha = (self.attractors_alpha + delta).max(0.0).min(1.0);
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_G: Scalar = 0.8;
        const MAX_ATTRACTORS: usize = 4;
        const MAX_MOVERS: usize = 32;
        let mut rng = SmallRng::from_entropy();
        let (width, height) = (state.width(), state.height());
        self.attractors = (0..MAX_ATTRACTORS)
            .map(|_| {
                     Attractor::new(state.random_color(Some(1.0)),
                                    rng.gen_range(width / 6.0, width * 5.0 / 6.0),
                                    rng.gen_range(height / 6.0, height * 5.0 / 6.0),
                                    rng.gen_range(10.0, 30.0),
                                    rng.gen_range(MAX_G / 4.0, MAX_G))
                 })
            .collect();
        self.movers = (0..MAX_MOVERS)
            .map(|_| {
                     Mover::new(state.random_color(Some(2.0 / 3.0)),
                                rng.gen_range(0.0, width),
                                rng.gen_range(0.0, height),
                                rng.gen_range(0.1, 4.2))
                 })
            .collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.handle_mouse(state);
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
                attractor.draw(context, gfx, self.attractors_alpha);
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

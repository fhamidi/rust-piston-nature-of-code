//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Oscillation - Forces and angular motion.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Attractor {
    color: Color,
    position: Vec2d,
    mass: Scalar,
    g: Scalar,
}

impl Attractor {
    fn new(color: Color, x: Scalar, y: Scalar) -> Self {
        Attractor {
            color: color,
            position: [x, y],
            mass: 20.0,
            g: 0.4,
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 2.0 + 4.0 * self.g)
            .resolution(self.mass as Resolution * 2)
            .color(self.color)
            .draw(
                ellipse::circle(self.position[0], self.position[1], self.mass * 2.0),
                &context.draw_state,
                context.transform,
                gfx,
            );
    }

    fn attract(&self, mover: &Mover) -> Vec2d {
        let force = vec2_sub(self.position, mover.position());
        let distance = vec2_len(force).max(5.0).min(25.0);
        vec2_scale(
            vec2_normalized(force),
            (self.g * self.mass * mover.mass()) / (distance * distance),
        )
    }
}

#[derive(Debug)]
struct Mover {
    color: Color,
    position: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
    angle: Scalar,
    angular_velocity: Scalar,
    angular_acceleration: Scalar,
    mass: Scalar,
}

impl Mover {
    fn new(color: Color, x: Scalar, y: Scalar, mass: Scalar) -> Self {
        let mut rng = thread_rng();
        let uniform = Uniform::new_inclusive(-1.0, 1.0);
        Mover {
            color: color,
            position: [x, y],
            velocity: [rng.sample(uniform), rng.sample(uniform)],
            acceleration: [0.0, 0.0],
            angle: 0.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
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
        let transform = context
            .transform
            .trans(self.position[0], self.position[1])
            .rot_rad(self.angle);
        Rectangle::new_border(color::BLACK, 1.0)
            .color(self.color)
            .draw(
                rectangle::centered_square(0.0, 0.0, self.mass * 16.0),
                &context.draw_state,
                transform,
                gfx,
            );
    }

    fn apply_force(&mut self, force: Vec2d) {
        self.acceleration =
            vec2_add(self.acceleration, vec2_scale(force, 1.0 / self.mass));
    }

    fn update(&mut self) {
        self.velocity = vec2_add(self.velocity, self.acceleration);
        self.position = vec2_add(self.position, self.velocity);
        self.angular_acceleration = self.acceleration[0] / 10.0;
        self.angular_velocity = (self.angular_velocity + self.angular_acceleration)
            .max(-0.1)
            .min(0.1);
        self.angle += self.angular_velocity;
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
        const MAX_MOVERS: usize = 16;
        let mut rng = thread_rng();
        let (width, height) = (state.width(), state.height());
        self.attractors.push(Attractor::new(
            state.random_color(Some(1.0)),
            width / 2.0,
            height / 2.0,
        ));
        self.movers = (0..MAX_MOVERS)
            .map(|_| {
                Mover::new(
                    state.random_color(Some(2.0 / 3.0)),
                    rng.gen_range(0.0, width),
                    rng.gen_range(0.0, height),
                    rng.gen_range(0.1, 2.0),
                )
            }).collect();
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

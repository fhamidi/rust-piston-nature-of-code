//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Forces - Basic application of gravity and wind.

extern crate piston_app;

use piston_app::*;

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
    fn velocity(&self) -> Vec2d {
        self.velocity
    }

    #[inline]
    fn mass(&self) -> Scalar {
        self.mass
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .resolution(self.mass as Resolution * 16)
            .color(self.color)
            .draw(
                ellipse::circle(self.position[0], self.position[1], self.mass * 8.0),
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
        let (x, y) = (self.position[0], self.position[1]);
        let (width, height) = (state.width(), state.height());
        if x > width || x < 0.0 {
            self.position[0] = x.max(0.0).min(width);
            self.velocity[0] *= -1.0;
        }
        if y > height || y < 0.0 {
            self.position[1] = y.max(0.0).min(height);
            self.velocity[1] *= -1.0;
        }
        self.velocity = vec2_add(self.velocity, self.acceleration);
        self.position = vec2_add(self.position, self.velocity);
        self.acceleration = [0.0, 0.0];
    }
}

#[derive(Debug)]
struct App {
    movers: Vec<Mover>,
}

impl App {
    fn new() -> Self {
        App { movers: vec![] }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_MOVERS: usize = 16;
        let mut rng = thread_rng();
        let uniform = Uniform::new(0.1, 5.0);
        self.movers = (0..MAX_MOVERS)
            .map(|_| {
                Mover::new(
                    state.random_color(Some(2.0 / 3.0)),
                    0.0,
                    0.0,
                    rng.sample(uniform),
                )
            })
            .collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        for mover in &mut self.movers {
            let gravity = [0.0, 0.1 * mover.mass()];
            let wind = [0.01, 0.0];
            mover.apply_force(gravity);
            mover.apply_force(wind);
            if state.mouse_button_pressed(MouseButton::Left) {
                const MU: Scalar = 0.1;
                let friction = vec2_scale(vec2_normalized(mover.velocity()), -1.0 * MU);
                mover.apply_force(friction);
            }
            mover.update(state);
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
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

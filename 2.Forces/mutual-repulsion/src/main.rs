//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Forces - Mutual gravitational repulsion.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Mover {
    color: Color,
    position: Vec2d,
    velocity: Vec2d,
    acceleration: Vec2d,
    mass: Scalar,
    g: Scalar,
}

impl Mover {
    fn new(color: Color, x: Scalar, y: Scalar, mass: Scalar, g: Scalar) -> Self {
        Mover {
            color: color,
            position: [x, y],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
            mass: mass,
            g: g,
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0 + 3.0 * self.g)
            .resolution(self.mass as Resolution * 12)
            .color(self.color)
            .draw(
                ellipse::circle(self.position[0], self.position[1], self.mass * 8.0),
                &context.draw_state,
                context.transform,
                gfx,
            );
    }

    fn attract(&self, other: &Self) -> Vec2d {
        let force = vec2_sub(self.position, other.position);
        let distance = vec2_len(force).max(1.0).min(27.0);
        vec2_scale(
            vec2_normalized(force),
            (self.g * self.mass * other.mass) / (distance * distance),
        )
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
            self.velocity[0] *= -0.42;
        }
        if y > height || y < 0.0 {
            self.position[1] = y.max(0.0).min(height);
            self.velocity[1] *= -0.42;
        }
        self.velocity = vec2_add(self.velocity, self.acceleration);
        self.position = vec2_add(self.position, self.velocity);
        self.acceleration = [0.0, 0.0];
    }
}

#[derive(Debug)]
struct App {
    attractor_color: Color,
    attractor_intensity: Scalar,
    movers: Vec<Mover>,
}

impl App {
    fn new() -> Self {
        App {
            attractor_color: color::TRANSPARENT,
            attractor_intensity: 0.0,
            movers: vec![],
        }
    }

    fn handle_mouse(&mut self, state: &PistonAppState) {
        let delta = if state.mouse_button_pressed(MouseButton::Left) {
            0.0042
        } else {
            -0.01
        };
        self.attractor_intensity = (self.attractor_intensity + delta).max(0.0).min(1.0);
        self.attractor_color = [
            self.attractor_color[0],
            self.attractor_color[1],
            self.attractor_color[2],
            self.attractor_intensity as ColorComponent,
        ];
    }
}

const MAX_G: Scalar = 0.8;
const MAX_MOVERS: usize = 12;

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        let mut rng = thread_rng();
        self.attractor_color =
            state.color_from_hsv(rng.gen_range(30.0, 90.0), 1.0, 1.0, 0.0);
        self.movers = (0..MAX_MOVERS)
            .map(|_| {
                Mover::new(
                    state.random_color(None),
                    rng.gen_range(0.0, state.width()),
                    rng.gen_range(0.0, state.height()),
                    rng.gen_range(3.0, 6.0),
                    rng.gen_range(MAX_G / 4.2, MAX_G),
                )
            })
            .collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        self.handle_mouse(state);
        let attractor = Mover::new(
            self.attractor_color,
            state.mouse_x(),
            state.mouse_y(),
            16.0 * self.attractor_intensity,
            MAX_G,
        );
        for i in 0..self.movers.len() {
            for j in 0..self.movers.len() {
                if i != j {
                    let force = self.movers[j].attract(&self.movers[i]);
                    self.movers[i].apply_force(vec2_scale(force, -1.0));
                }
                let force = attractor.attract(&self.movers[i]);
                self.movers[i].apply_force(force);
            }
            self.movers[i].update(state);
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            if self.attractor_intensity > 0.01 {
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

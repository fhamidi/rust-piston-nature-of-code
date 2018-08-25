//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Forces - Helium ballon with wall bouncing and optional wind.

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
        let mut rng = thread_rng();
        Mover {
            color: state.random_color(Some(2.0 / 3.0)),
            position: [rng.gen_range(0.0, state.width()),
                       rng.gen_range(state.height() * 4.0 / 5.0, state.height())],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .resolution(20)
            .color(self.color)
            .draw(ellipse::circle(self.position[0], self.position[1], 32.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn apply_force(&mut self, force: Vec2d) {
        self.acceleration = vec2_add(self.acceleration, force);
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
        self.apply_force([0.0, -0.1]);
        self.velocity = vec2_add(self.velocity, self.acceleration);
        self.position = vec2_add(self.position, self.velocity);
        self.acceleration = [0.0, 0.0];
    }
}

#[derive(Debug)]
struct App {
    movers: Vec<Mover>,
    wind: Vec2d,
    wind_offset: Scalar,
}

const MAX_WIND: Scalar = 2.0 / 3.0;

impl App {
    fn new() -> Self {
        App {
            movers: vec![],
            wind: [0.0, 0.0],
            wind_offset: 0.0,
        }
    }

    fn draw_wind(&mut self, context: Context, gfx: &mut G2d, state: &PistonAppState) {
        let wind = vec2_len(self.wind);
        if wind > 0.01 {
            let hue = 240.0 - state.map_range(wind, 0.0, MAX_WIND, 0.0, 240.0);
            let color = state.color_from_hsv(hue, 1.0, 2.0 / 3.0, 1.0);
            let wind_len = state.width() * 0.42;
            let arrow =
                [0.0,
                 0.0,
                 state.map_range(self.wind[0], -MAX_WIND, MAX_WIND, -wind_len, wind_len),
                 0.0];
            let transform = context
                .transform
                .trans(state.width() / 2.0, state.height() - 32.0);
            Line::new_round(color, 2.0)
                .draw_arrow(arrow, 4.2, &context.draw_state, transform, gfx);
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_MOVERS: usize = 32;
        self.movers = (0..MAX_MOVERS).map(|_| Mover::new(state)).collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        if state.mouse_button_pressed(MouseButton::Left) {
            let mut wind =
                state
                    .map_range(state.noise(&[self.wind_offset]), 0.0, 1.0, 0.0, MAX_WIND);
            if state.mouse_x() < state.width() / 2.0 {
                wind = -wind;
            }
            self.wind = [wind, 0.0];
        } else {
            self.wind = [0.0, 0.0];
        }
        self.wind_offset += 0.01;
        for mover in &mut self.movers {
            mover.apply_force(self.wind);
            mover.update(state);
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            for mover in &self.movers {
                mover.draw(context, gfx);
            }
            self.draw_wind(context, gfx, state);
        });
    }
}

fn main() {
    let mut app = App::new();
    App::run(env!("CARGO_PKG_NAME"), &mut app);
}

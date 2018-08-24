//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Forces - Fluid resistance.

extern crate piston_app;

use piston_app::*;

#[derive(Debug)]
struct Liquid {
    rect: types::Rectangle,
    drag_coeff: Scalar,
}

impl Liquid {
    fn new<R: Into<types::Rectangle>>(rect: R, drag_coeff: Scalar) -> Self {
        Liquid {
            rect: rect.into(),
            drag_coeff: drag_coeff,
        }
    }

    #[inline]
    fn rect(&self) -> types::Rectangle {
        self.rect
    }

    #[inline]
    fn drag_coeff(&self) -> Scalar {
        self.drag_coeff
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        rectangle([0.0, 0.0, 1.0 / 3.0, 1.0],
                  self.rect,
                  context.transform,
                  gfx);
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

    fn mass(&self) -> Scalar {
        self.mass
    }

    fn is_inside(&self, liquid: &Liquid) -> bool {
        let (x, y) = (self.position[0], self.position[1]);
        let ref rect = liquid.rect();
        x > rect[0] && x < rect[0] + rect[2] && y > rect[1] && y < rect[1] + rect[3]
    }

    fn draw(&self, context: Context, gfx: &mut G2d) {
        Ellipse::new_border(color::BLACK, 1.0)
            .resolution(self.mass as Resolution * 16)
            .color(self.color)
            .draw(ellipse::circle(self.position[0], self.position[1], self.mass * 8.0),
                  &context.draw_state,
                  context.transform,
                  gfx);
    }

    fn apply_drag(&mut self, liquid: &Liquid) {
        let drag = vec2_scale(vec2_normalized(self.velocity),
                              -1.0 * liquid.drag_coeff() *
                              vec2_square_len(self.velocity));
        self.apply_force(drag);
    }

    fn apply_force(&mut self, force: Vec2d) {
        self.acceleration = vec2_add(self.acceleration,
                                     vec2_scale(force, 1.0 / self.mass));
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
    liquids: Vec<Liquid>,
    movers: Vec<Mover>,
}

impl App {
    fn new() -> Self {
        App {
            liquids: vec![],
            movers: vec![],
        }
    }
}

impl PistonApp for App {
    fn setup(&mut self, _: &mut PistonAppWindow, state: &PistonAppState) {
        const MAX_MOVERS: usize = 16;
        let (width, height) = (state.width(), state.height());
        self.liquids
            .push(Liquid::new([0.0, height / 2.0, width, height / 2.0], 0.1));
        let gap = width / MAX_MOVERS as Scalar;
        let mut rng = SmallRng::from_entropy();
        self.movers = (0..MAX_MOVERS)
            .map(|i| {
                     Mover::new(state.random_color(Some(1.0)),
                                i as Scalar * gap + gap / 2.0,
                                rng.gen_range(0.0, height / 4.0),
                                rng.gen_range(0.1, 5.0))
                 })
            .collect();
    }

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState) {
        for mover in &mut self.movers {
            let gravity = [0.0, 0.1 * mover.mass()];
            mover.apply_force(gravity);
            for liquid in &self.liquids {
                if mover.is_inside(liquid) {
                    mover.apply_drag(liquid);
                }
            }
            mover.update(state);
        }
        window.draw_2d(state.event(), |context, gfx| {
            clear(color::WHITE, gfx);
            for liquid in &self.liquids {
                liquid.draw(context, gfx);
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

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Simple application framework, similar to the Processing environment
//! used in the book.

extern crate noise;
extern crate piston_window;
extern crate rand;
extern crate sdl2_window;

pub use math::*;
pub use piston_window::*;
pub use rand::Rng;
pub use types::{Color, ColorComponent};

const MIN_COLOR_COMPONENT: ColorComponent = 1.0 / 3.0;

pub struct PistonAppState {
    mouse_button: MouseButton,
    mouse_pressed: u8,
    mouse_x: Scalar,
    mouse_y: Scalar,
    noise_seed: noise::Seed,
    width: Scalar,
    height: Scalar,
}

impl PistonAppState {
    fn new() -> Self {
        PistonAppState {
            mouse_button: MouseButton::Unknown,
            mouse_pressed: 0,
            mouse_x: 0.0,
            mouse_y: 0.0,
            noise_seed: rand::random::<noise::Seed>(),
            width: 0.0,
            height: 0.0,
        }
    }

    #[inline]
    pub fn mouse_button(&self) -> MouseButton {
        self.mouse_button
    }

    #[inline]
    pub fn mouse_pressed(&self) -> bool {
        self.mouse_pressed > 0
    }

    #[inline]
    pub fn mouse_x(&self) -> Scalar {
        self.mouse_x
    }

    #[inline]
    pub fn mouse_y(&self) -> Scalar {
        self.mouse_y
    }

    #[inline]
    pub fn width(&self) -> Scalar {
        self.width
    }

    #[inline]
    pub fn height(&self) -> Scalar {
        self.height
    }

    #[inline]
    pub fn map_x(&self, x: Scalar) -> Scalar {
        self.map_range(x, 0.0, 1.0, 0.0, self.width())
    }

    #[inline]
    pub fn map_y(&self, y: Scalar) -> Scalar {
        self.map_range(y, 0.0, 1.0, 0.0, self.height())
    }

    #[inline]
    pub fn map_range(&self,
                     value: Scalar,
                     in_min: Scalar,
                     in_max: Scalar,
                     out_min: Scalar,
                     out_max: Scalar)
                     -> Scalar {
        (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
    }

    pub fn noise(&self, input: Scalar) -> Scalar {
        (noise::perlin2(&self.noise_seed, &[input, 0.0]) + 1.0) / 2.0
    }

    pub fn random_color(&self, alpha: Option<ColorComponent>) -> Color {
        let mut rng = rand::thread_rng();
        [rng.gen_range(MIN_COLOR_COMPONENT, 1.0),
         rng.gen_range(MIN_COLOR_COMPONENT, 1.0),
         rng.gen_range(MIN_COLOR_COMPONENT, 1.0),
         alpha.unwrap_or_else(|| rng.gen_range(MIN_COLOR_COMPONENT, 1.0))]
    }

    pub fn noise_color(&self, input: Scalar, alpha: Option<ColorComponent>) -> Color {
        let alpha = alpha.unwrap_or_else(|| {
            self.map_range(self.noise(input),
                           0.0,
                           1.0,
                           MIN_COLOR_COMPONENT as Scalar,
                           1.0) as ColorComponent
        });
        hsv([1.0, 0.0, 0.0, alpha],
            self.map_range(self.noise(input + 25.0),
                           0.0,
                           1.0,
                           0.0,
                           2.0 * ::std::f64::consts::PI) as ColorComponent,
            self.noise(input + 50.0) as ColorComponent,
            self.map_range(self.noise(input + 75.0),
                           0.0,
                           1.0,
                           2.0 * MIN_COLOR_COMPONENT as Scalar,
                           1.0) as ColorComponent)
    }
}

pub trait PistonApp {
    fn setup(&mut self, _context: Context, _gl: &mut G2d, _state: &PistonAppState) {}

    fn draw(&mut self, context: Context, gl: &mut G2d, state: &PistonAppState);

    fn run<T: Into<String>>(title: T, app: &mut Self) {
        let mut first = true;
        let mut state = PistonAppState::new();
        let mut window: PistonWindow<sdl2_window::Sdl2Window> =
            WindowSettings::new(title, [640, 480])
                .exit_on_esc(true)
                .resizable(false)
                .build()
                .unwrap();
        while let Some(e) = window.next() {
            if let Some(args) = e.render_args() {
                state.width = args.width as Scalar;
                state.height = args.height as Scalar;
                window.draw_2d(&e, |context, gl| {
                    if first {
                        first = false;
                        app.setup(context, gl, &state);
                    }
                    app.draw(context, gl, &state);
                });
            }
            if let Some(Button::Mouse(_)) = e.press_args() {
                state.mouse_pressed += 1;
            }
            if let Some(Button::Mouse(_)) = e.release_args() {
                if state.mouse_pressed > 0 {
                    state.mouse_pressed -= 1;
                }
            }
            if let Some(position) = e.mouse_cursor_args() {
                state.mouse_x = position[0];
                state.mouse_y = position[1];
            }
        }
    }
}

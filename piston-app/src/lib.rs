//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Simple application framework, similar to the Processing environment
//! used in the book.

extern crate fnv;
extern crate fps_counter;
extern crate noise;
extern crate piston_window;
extern crate rand;
extern crate sdl2_window;
extern crate vecmath;

pub use std::f64::consts;

pub use math::{Scalar, Vec2d, Matrix2d};
pub use piston_window::*;
pub use rand::distributions::normal::StandardNormal;
pub use rand::distributions::uniform::Uniform;
pub use rand::prelude::*;
pub use types::{Color, ColorComponent, Resolution};
pub use vecmath::*;

use fnv::*;
use fps_counter::*;
use noise::{NoiseFn, Perlin, Seedable};

pub type PistonAppWindow = PistonWindow<sdl2_window::Sdl2Window>;

pub trait PistonApp {
    fn setup(&mut self, _window: &mut PistonAppWindow, _state: &PistonAppState) {}

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState);

    fn run<T: Into<String>>(title: T, app: &mut Self) {
        let title = title.into();
        let mut first = true;
        let mut state = PistonAppState::new();
        let mut window: PistonAppWindow = WindowSettings::new(title.clone(), [640, 480])
            .exit_on_esc(true)
            .resizable(false)
            .vsync(true)
            .build()
            .unwrap();
        let mut fps = FPSCounter::new();
        while let Some(e) = window.next() {
            if let Some(args) = e.render_args() {
                state.event = e.clone();
                state.viewport = args.viewport();
                if first {
                    first = false;
                    app.setup(&mut window, &state);
                }
                app.draw(&mut window, &state);
                state.frame_count += 1;
                state.clicked_mouse_buttons.clear();
                window.set_title(format!("{} ({} FPS)", title, fps.tick()));
            }
            if let Some(position) = e.mouse_cursor_args() {
                state.mouse_x = position[0];
                state.mouse_y = position[1];
            }
            match e.press_args() {
                Some(Button::Keyboard(key)) => {
                    state.pressed_keys.insert(key);
                }
                Some(Button::Mouse(button)) => {
                    state.pressed_mouse_buttons.insert(button);
                }
                _ => (),
            }
            match e.release_args() {
                Some(Button::Keyboard(key)) => {
                    state.pressed_keys.remove(&key);
                }
                Some(Button::Mouse(button)) => {
                    if state.pressed_mouse_buttons.contains(&button) {
                        state.pressed_mouse_buttons.remove(&button);
                        state.clicked_mouse_buttons.insert(button);
                    }
                }
                _ => (),
            }
        }
    }
}

pub struct PistonAppState {
    event: Event,
    viewport: Viewport,
    frame_count: usize,
    pressed_keys: FnvHashSet<Key>,
    pressed_mouse_buttons: FnvHashSet<MouseButton>,
    clicked_mouse_buttons: FnvHashSet<MouseButton>,
    mouse_x: Scalar,
    mouse_y: Scalar,
    noise: Perlin,
}

impl PistonAppState {
    fn new() -> Self {
        PistonAppState {
            event: Event::Loop(Loop::Render(RenderArgs {
                                                ext_dt: 0.0,
                                                width: 0,
                                                height: 0,
                                                draw_width: 0,
                                                draw_height: 0,
                                            })),
            viewport: Viewport {
                rect: [0, 0, 0, 0],
                draw_size: [0, 0],
                window_size: [0, 0],
            },
            frame_count: 0,
            pressed_keys: Default::default(),
            pressed_mouse_buttons: Default::default(),
            clicked_mouse_buttons: Default::default(),
            mouse_x: 0.0,
            mouse_y: 0.0,
            noise: Perlin::new().set_seed(thread_rng().gen()),
        }
    }

    #[inline]
    pub fn event(&self) -> &Event {
        &self.event
    }

    #[inline]
    pub fn viewport(&self) -> Viewport {
        self.viewport
    }

    #[inline]
    pub fn width(&self) -> Scalar {
        self.viewport.draw_size[0] as Scalar
    }

    #[inline]
    pub fn height(&self) -> Scalar {
        self.viewport.draw_size[1] as Scalar
    }

    #[inline]
    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    #[inline]
    pub fn key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    #[inline]
    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    #[inline]
    pub fn mouse_button_clicked(&self, button: MouseButton) -> bool {
        self.clicked_mouse_buttons.contains(&button)
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
    pub fn map_range(&self,
                     value: Scalar,
                     in_min: Scalar,
                     in_max: Scalar,
                     out_min: Scalar,
                     out_max: Scalar)
                     -> Scalar {
        (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
    }

    #[inline]
    pub fn map_x(&self, x: Scalar) -> Scalar {
        self.map_range(x, 0.0, 1.0, 0.0, self.width())
    }

    #[inline]
    pub fn map_y(&self, y: Scalar) -> Scalar {
        self.map_range(y, 0.0, 1.0, 0.0, self.height())
    }

    pub fn noise(&self, input: &[Scalar]) -> Scalar {
        self.map_range(match input.len() {
                           0 => 0.0,
                           1 => self.noise.get([input[0], 0.0]),
                           2 => self.noise.get([input[0], input[1]]),
                           3 => self.noise.get([input[0], input[1], input[2]]),
                           _ => self.noise.get([input[0], input[1], input[2], input[3]]),
                       },
                       -1.0,
                       1.0,
                       0.0,
                       1.0)
    }

    pub fn noise_color(&self,
                       base_hue: Scalar,
                       offset: Scalar,
                       alpha: Option<ColorComponent>)
                       -> Color {
        const MIN_ALPHA: Scalar = 1.0 / 3.0;
        const MIN_SATURATION: Scalar = 0.5;
        const MIN_VALUE: Scalar = 0.5;
        let alpha = alpha.unwrap_or_else(|| {
            self.map_range(self.noise.get([offset - 29.0, 0.0]).abs(),
                           0.0,
                           1.0,
                           MIN_ALPHA,
                           1.0) as ColorComponent
        });
        let base_hue = base_hue * 360.0;
        self.color_from_hsv(self.map_range(self.noise.get([offset, 0.0]).abs(),
                                           0.0,
                                           1.0,
                                           base_hue,
                                           base_hue + 360.0) %
                            360.0,
                            self.map_range(self.noise.get([offset + 17.0, 0.0]).abs(),
                                           0.0,
                                           1.0,
                                           MIN_SATURATION,
                                           1.0),
                            self.map_range(self.noise.get([offset - 43.0, 0.0]).abs(),
                                           0.0,
                                           1.0,
                                           MIN_VALUE,
                                           1.0),
                            alpha)
    }

    pub fn random_color(&self, alpha: Option<ColorComponent>) -> Color {
        const MIN_COLOR_COMPONENT: ColorComponent = 1.0 / 3.0;
        let mut rng = thread_rng();
        let uniform = Uniform::new_inclusive(MIN_COLOR_COMPONENT, 1.0);
        [rng.sample(uniform),
         rng.sample(uniform),
         rng.sample(uniform),
         alpha.unwrap_or_else(|| rng.sample(uniform))]
    }

    pub fn color_from_hsv(&self,
                          hue: Scalar,
                          saturation: Scalar,
                          value: Scalar,
                          alpha: ColorComponent)
                          -> Color {
        let c = value * saturation;
        let h = (hue - ((hue / 360.0).floor() * 360.0)) / 60.0;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
        let m = value - c;
        let (r, g, b) = match h {
            h if h >= 0.0 && h <= 1.0 => (c, x, 0.0),
            h if h >= 1.0 && h <= 2.0 => (x, c, 0.0),
            h if h >= 2.0 && h <= 3.0 => (0.0, c, x),
            h if h >= 3.0 && h <= 4.0 => (0.0, x, c),
            h if h >= 4.0 && h <= 5.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        [(r + m) as ColorComponent,
         (g + m) as ColorComponent,
         (b + m) as ColorComponent,
         alpha]
    }

    pub fn draw_centered_texture<G: Graphics>(&self,
                                              texture: &G::Texture,
                                              color: Option<Color>,
                                              x: Scalar,
                                              y: Scalar,
                                              draw_state: &DrawState,
                                              transform: Matrix2d,
                                              gfx: &mut G) {
        let (width, height) = texture.get_size();
        let half_width = width as Scalar / 2.0;
        let half_height = height as Scalar / 2.0;
        Image::new()
            .maybe_color(color)
            .rect(rectangle::centered([x, y, half_width, half_height]))
            .draw(texture, draw_state, transform, gfx);
    }
}

pub fn vec2_heading(vec: Vec2d) -> Scalar {
    vec[1].atan2(vec[0])
}

pub fn vec2_limit(vec: Vec2d, max: Scalar) -> Vec2d {
    if vec2_len(vec) > max {
        vec2_scale(vec2_normalized(vec), max)
    } else {
        vec
    }
}

pub fn vec2_random() -> Vec2d {
    let mut rng = thread_rng();
    let uniform = Uniform::new_inclusive(-1.0, 1.0);
    vec2_normalized([rng.sample(uniform), rng.sample(uniform)])
}

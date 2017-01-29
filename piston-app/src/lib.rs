//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Simple application framework, similar to the Processing environment
//! used in the book.

pub extern crate image as img;
pub extern crate rand;

extern crate noise;
extern crate piston_window;
extern crate sdl2_window;
extern crate vecmath;

pub use math::{Scalar, Vec2d};
pub use piston_window::*;
pub use rand::Rng;
pub use rand::distributions::normal::StandardNormal;
pub use types::{Color, ColorComponent};
pub use vecmath::*;

pub type PistonAppWindow = PistonWindow<sdl2_window::Sdl2Window>;

pub trait PistonApp {
    fn setup(&mut self, _window: &mut PistonAppWindow, _state: &PistonAppState) {}

    fn draw(&mut self, window: &mut PistonAppWindow, state: &PistonAppState);

    fn run<T: Into<String>>(title: T, app: &mut Self) {
        let mut first = true;
        let mut state = PistonAppState::new();
        let mut window: PistonAppWindow = WindowSettings::new(title, [640, 480])
            .exit_on_esc(true)
            .resizable(false)
            .vsync(true)
            .build()
            .unwrap();
        while let Some(e) = window.next() {
            if let Some(args) = e.render_args() {
                state.event = e.clone();
                state.width = args.width as Scalar;
                state.height = args.height as Scalar;
                if first {
                    first = false;
                    app.setup(&mut window, &state);
                }
                app.draw(&mut window, &state);
            }
            if let Some(position) = e.mouse_cursor_args() {
                state.mouse_x = position[0];
                state.mouse_y = position[1];
            }
            match e.press_args() {
                Some(Button::Keyboard(key)) => {
                    state.key = key;
                    state.key_pressed += 1;
                }
                Some(Button::Mouse(button)) => {
                    state.mouse_button = button;
                    state.mouse_pressed += 1;
                }
                _ => (),
            }
            match e.release_args() {
                Some(Button::Keyboard(_)) if state.key_pressed > 0 => {
                    state.key_pressed -= 1;
                }
                Some(Button::Mouse(_)) if state.mouse_pressed > 0 => {
                    state.mouse_pressed -= 1;
                }
                _ => (),
            }
        }
    }
}

pub struct PistonAppState {
    event: Input,
    width: Scalar,
    height: Scalar,
    key: Key,
    key_pressed: u8,
    mouse_button: MouseButton,
    mouse_pressed: u8,
    mouse_x: Scalar,
    mouse_y: Scalar,
    noise_seed: noise::Seed,
}

impl PistonAppState {
    fn new() -> Self {
        PistonAppState {
            event: Input::Render(RenderArgs {
                ext_dt: 0.0,
                width: 0,
                height: 0,
                draw_width: 0,
                draw_height: 0,
            }),
            width: 0.0,
            height: 0.0,
            key: Key::Unknown,
            key_pressed: 0,
            mouse_button: MouseButton::Unknown,
            mouse_pressed: 0,
            mouse_x: 0.0,
            mouse_y: 0.0,
            noise_seed: rand::random(),
        }
    }

    #[inline]
    pub fn event(&self) -> &Input {
        &self.event
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
    pub fn key(&self) -> Key {
        self.key
    }

    #[inline]
    pub fn key_pressed(&self) -> bool {
        self.key_pressed > 0
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

    pub fn noise(&self, input: &[Scalar]) -> Scalar {
        let result = match input.len() {
            0 => 0.0,
            1 => noise::perlin2(&self.noise_seed, &[input[0], 0.0]),
            2 => noise::perlin2(&self.noise_seed, &[input[0], input[1]]),
            3 => noise::perlin3(&self.noise_seed, &[input[0], input[1], input[2]]),
            _ => {
                noise::perlin4(&self.noise_seed,
                               &[input[0], input[1], input[2], input[3]])
            }
        };
        ((result + 1.0) / 2.0).max(0.0).min(1.0)
    }

    pub fn random_color(&self, alpha: Option<ColorComponent>) -> Color {
        const MIN_COLOR_COMPONENT: ColorComponent = 1.0 / 3.0;
        let mut rng = rand::thread_rng();
        [rng.gen_range(MIN_COLOR_COMPONENT, 1.0),
         rng.gen_range(MIN_COLOR_COMPONENT, 1.0),
         rng.gen_range(MIN_COLOR_COMPONENT, 1.0),
         alpha.unwrap_or_else(|| rng.gen_range(MIN_COLOR_COMPONENT, 1.0))]
    }

    pub fn noise_color(&self, input: Scalar, alpha: Option<ColorComponent>) -> Color {
        const MIN_ALPHA: Scalar = 1.0 / 3.0;
        const MIN_SATURATION: Scalar = 1.0 / 2.0;
        const MIN_VALUE: Scalar = 2.0 / 3.0;
        let alpha = alpha.unwrap_or_else(|| {
            self.map_range(self.noise(&[input]),
                           0.0,
                           1.0,
                           MIN_ALPHA,
                           1.0) as ColorComponent
        });
        self.color_from_hsv(self.map_range(self.noise(&[input + 25.0]),
                                           0.0,
                                           1.0,
                                           0.0,
                                           360.0),
                            self.map_range(self.noise(&[input + 50.0]),
                                           0.0,
                                           1.0,
                                           MIN_SATURATION,
                                           1.0),
                            self.map_range(self.noise(&[input + 75.0]),
                                           0.0,
                                           1.0,
                                           MIN_VALUE,
                                           1.0),
                            alpha)
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
            0.0...1.0 => (c, x, 0.0),
            1.0...2.0 => (x, c, 0.0),
            2.0...3.0 => (0.0, c, x),
            3.0...4.0 => (0.0, x, c),
            4.0...5.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        [(r + m) as ColorComponent,
         (g + m) as ColorComponent,
         (b + m) as ColorComponent,
         alpha]
    }
}

pub struct TextureCanvas {
    canvas: img::RgbaImage,
    texture: G2dTexture,
}

impl TextureCanvas {
    pub fn new(window: &mut PistonAppWindow,
               width: Scalar,
               height: Scalar,
               settings: Option<TextureSettings>)
               -> Self {
        let canvas = img::RgbaImage::new(width as u32, height as u32);
        let texture = Texture::from_image(&mut window.factory,
                                          &canvas,
                                          &settings.unwrap_or(TextureSettings::new()))
            .unwrap();
        TextureCanvas {
            canvas: canvas,
            texture: texture,
        }
    }

    #[inline]
    pub fn canvas(&self) -> &img::RgbaImage {
        &self.canvas
    }

    #[inline]
    pub fn canvas_mut(&mut self) -> &mut img::RgbaImage {
        &mut self.canvas
    }

    #[inline]
    pub fn texture(&self) -> &G2dTexture {
        &self.texture
    }

    pub fn update<F>(&mut self, window: &mut PistonAppWindow, f: F)
        where F: FnOnce(&mut img::RgbaImage)
    {
        f(&mut self.canvas);
        self.texture.update(&mut window.encoder, &self.canvas).unwrap();
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
    let mut rng = rand::thread_rng();
    vec2_normalized([rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0)])
}

//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Simple application framework, similar to the Processing environment
//! used in the book.

pub extern crate gfx;

extern crate fnv;
extern crate fps_counter;
extern crate gfx_device_gl;
extern crate noise;
extern crate piston_window;
extern crate rand;
extern crate sdl2_window;
extern crate serde_json;
extern crate shaders_graphics2d;
extern crate vecmath;

pub use std::f64::consts;

pub use gfx::*;
pub use math::{Matrix2d, Scalar, Vec2d};
pub use piston_window::*;
pub use rand::distributions::normal::StandardNormal;
pub use rand::distributions::uniform::Uniform;
pub use rand::prelude::*;
pub use types::{Color, ColorComponent, Resolution};
pub use vecmath::*;

use std::collections::hash_set;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use fnv::*;
use fps_counter::*;
use gfx::traits::FactoryExt;
use gfx_device_gl::Resources;
use noise::{NoiseFn, Perlin, Seedable};
use shaders_graphics2d::{colored, textured};

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
                state.hit_keys.clear();
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
                    if state.pressed_keys.contains(&key) {
                        state.pressed_keys.remove(&key);
                        state.hit_keys.insert(key);
                    }
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
    hit_keys: FnvHashSet<Key>,
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
            hit_keys: Default::default(),
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
    pub fn any_key_pressed(&self) -> bool {
        self.pressed_keys.len() > 0
    }

    #[inline]
    pub fn key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    #[inline]
    pub fn pressed_keys(&self) -> hash_set::Iter<Key> {
        self.pressed_keys.iter()
    }

    #[inline]
    pub fn any_key_hit(&self) -> bool {
        self.hit_keys.len() > 0
    }

    #[inline]
    pub fn key_hit(&self, key: Key) -> bool {
        self.hit_keys.contains(&key)
    }

    #[inline]
    pub fn hit_keys(&self) -> hash_set::Iter<Key> {
        self.hit_keys.iter()
    }

    #[inline]
    pub fn any_mouse_button_pressed(&self) -> bool {
        self.pressed_mouse_buttons.len() > 0
    }

    #[inline]
    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    #[inline]
    pub fn pressed_mouse_buttons(&self) -> hash_set::Iter<MouseButton> {
        self.pressed_mouse_buttons.iter()
    }

    #[inline]
    pub fn any_mouse_button_clicked(&self) -> bool {
        self.clicked_mouse_buttons.len() > 0
    }

    #[inline]
    pub fn mouse_button_clicked(&self, button: MouseButton) -> bool {
        self.clicked_mouse_buttons.contains(&button)
    }

    #[inline]
    pub fn clicked_mouse_buttons(&self) -> hash_set::Iter<MouseButton> {
        self.clicked_mouse_buttons.iter()
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
    pub fn map_range(
        &self,
        value: Scalar,
        in_min: Scalar,
        in_max: Scalar,
        out_min: Scalar,
        out_max: Scalar,
    ) -> Scalar {
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

    #[inline]
    pub fn normalize_x(&self, x: Scalar) -> Scalar {
        self.map_range(x, 0.0, self.width(), -1.0, 1.0)
    }

    #[inline]
    pub fn normalize_y(&self, y: Scalar) -> Scalar {
        -self.map_range(y, 0.0, self.height(), -1.0, 1.0)
    }

    pub fn noise(&self, input: &[Scalar]) -> Scalar {
        self.map_range(
            match input.len() {
                0 => 0.0,
                1 => self.noise.get([input[0], 0.0]),
                2 => self.noise.get([input[0], input[1]]),
                3 => self.noise.get([input[0], input[1], input[2]]),
                _ => self.noise.get([input[0], input[1], input[2], input[3]]),
            },
            -1.0,
            1.0,
            0.0,
            1.0,
        )
    }

    pub fn noise_color(
        &self,
        base_hue: Scalar,
        offset: Scalar,
        alpha: Option<ColorComponent>,
    ) -> Color {
        const MIN_ALPHA: Scalar = 1.0 / 3.0;
        const MIN_SATURATION: Scalar = 0.5;
        const MIN_VALUE: Scalar = 0.5;
        let alpha = alpha.unwrap_or_else(|| {
            self.map_range(
                self.noise.get([offset - 29.0, 0.0]).abs(),
                0.0,
                1.0,
                MIN_ALPHA,
                1.0,
            ) as ColorComponent
        });
        let base_hue = base_hue * 360.0;
        self.color_from_hsv(
            self.map_range(
                self.noise.get([offset, 0.0]).abs(),
                0.0,
                1.0,
                base_hue,
                base_hue + 360.0,
            ) % 360.0,
            self.map_range(
                self.noise.get([offset + 17.0, 0.0]).abs(),
                0.0,
                1.0,
                MIN_SATURATION,
                1.0,
            ),
            self.map_range(
                self.noise.get([offset - 43.0, 0.0]).abs(),
                0.0,
                1.0,
                MIN_VALUE,
                1.0,
            ),
            alpha,
        )
    }

    pub fn random_color(&self, alpha: Option<ColorComponent>) -> Color {
        const MIN_COLOR_COMPONENT: ColorComponent = 1.0 / 3.0;
        let mut rng = thread_rng();
        let uniform = Uniform::new_inclusive(MIN_COLOR_COMPONENT, 1.0);
        [
            rng.sample(uniform),
            rng.sample(uniform),
            rng.sample(uniform),
            alpha.unwrap_or_else(|| rng.sample(uniform)),
        ]
    }

    pub fn color_from_hsv(
        &self,
        hue: Scalar,
        saturation: Scalar,
        value: Scalar,
        alpha: ColorComponent,
    ) -> Color {
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
        [
            (r + m) as ColorComponent,
            (g + m) as ColorComponent,
            (b + m) as ColorComponent,
            alpha,
        ]
    }

    pub fn draw_centered_texture(
        &self,
        texture: &G2dTexture,
        color: Option<Color>,
        x: Scalar,
        y: Scalar,
        draw_state: &DrawState,
        transform: Matrix2d,
        gfx: &mut G2d,
    ) {
        let (width, height) = texture.get_size();
        let half_width = width as Scalar / 2.0;
        let half_height = height as Scalar / 2.0;
        Image::new()
            .maybe_color(color)
            .rect(rectangle::centered([x, y, half_width, half_height]))
            .draw(texture, draw_state, transform, gfx);
    }
}

pub type PistonPipeline<M> = pso::PipelineState<Resources, M>;

pub type PistonPipelineSampler = (
    gfx::handle::ShaderResourceView<Resources, [f32; 4]>,
    gfx::handle::Sampler<Resources>,
);

#[derive(Debug)]
pub struct PistonPipelineBuilder {
    texture_atlas: Option<TextureAtlas>,
    vertex_shader: Option<&'static [u8]>,
    fragment_shader: Option<&'static [u8]>,
}

impl PistonPipelineBuilder {
    pub fn new() -> Self {
        PistonPipelineBuilder {
            texture_atlas: None,
            vertex_shader: None,
            fragment_shader: None,
        }
    }

    pub fn texture_atlas(mut self, texture_atlas: TextureAtlas) -> Self {
        self.texture_atlas = Some(texture_atlas);
        self
    }

    pub fn vertex_shader(mut self, bytes: &'static [u8]) -> Self {
        self.vertex_shader = Some(bytes);
        self
    }

    pub fn fragment_shader(mut self, bytes: &'static [u8]) -> Self {
        self.fragment_shader = Some(bytes);
        self
    }

    pub fn build<I: pso::PipelineInit>(
        self,
        window: &mut PistonAppWindow,
        init: I,
    ) -> Result<(PistonPipeline<I::Meta>, PistonRenderer), Box<Error>> {
        let factory = &mut window.factory;
        let mut default_vertex_shader = colored::VERTEX_GLSL_150_CORE;
        let mut default_fragment_shader = colored::FRAGMENT_GLSL_150_CORE;
        if self.texture_atlas.is_some() {
            default_vertex_shader = textured::VERTEX_GLSL_150_CORE;
            default_fragment_shader = textured::FRAGMENT_GLSL_150_CORE;
        }
        Ok((
            factory.create_pipeline_simple(
                self.vertex_shader.unwrap_or(default_vertex_shader),
                self.fragment_shader.unwrap_or(default_fragment_shader),
                init,
            )?,
            PistonRenderer {
                texture_atlas: self.texture_atlas,
            },
        ))
    }
}

#[derive(Debug)]
pub struct PistonRenderer {
    texture_atlas: Option<TextureAtlas>,
}

impl PistonRenderer {
    #[inline]
    pub fn texture_atlas(&self) -> Option<&TextureAtlas> {
        self.texture_atlas.as_ref()
    }

    pub fn clear(&self, window: &mut PistonAppWindow, color: Color) {
        window.encoder.clear(&window.output_color, color);
    }

    pub fn draw<B, D, F, V>(
        &self,
        window: &mut PistonAppWindow,
        pipeline: &PistonPipeline<D::Meta>,
        vertices: &[V],
        indices: B,
        f: F,
    ) where
        B: IntoIndexBuffer<Resources>,
        D: pso::PipelineData<Resources>,
        F: FnOnce(
            gfx::handle::Buffer<Resources, V>,
            gfx::handle::RenderTargetView<Resources, gfx::format::Srgba8>,
        ) -> D,
        V: gfx::traits::Pod + pso::buffer::Structure<gfx::format::Format>,
    {
        let (vbuf, slice) = window
            .factory
            .create_vertex_buffer_with_slice(vertices, indices);
        let data = f(vbuf, window.output_color.clone());
        let encoder = &mut window.encoder;
        encoder.draw(&slice, pipeline, &data);
        encoder.flush(&mut window.device);
    }
}

#[derive(Debug)]
pub struct TextureAtlas {
    texture: G2dTexture,
    atlas: Vec<[Scalar; 4]>,
    normalized_atlas: Vec<[f32; 4]>,
}

impl TextureAtlas {
    pub fn from_path<P: AsRef<Path>>(
        window: &mut PistonAppWindow,
        texture_path: P,
    ) -> Result<Self, Box<Error>> {
        Self::from_maybe_paths(window, texture_path, None)
    }

    pub fn from_paths<P: AsRef<Path>>(
        window: &mut PistonAppWindow,
        texture_path: P,
        atlas_path: P,
    ) -> Result<Self, Box<Error>> {
        Self::from_maybe_paths(window, texture_path, Some(atlas_path))
    }

    pub fn from_maybe_paths<P: AsRef<Path>>(
        window: &mut PistonAppWindow,
        texture_path: P,
        atlas_path: Option<P>,
    ) -> Result<Self, Box<Error>> {
        let texture = Texture::from_path(
            &mut window.factory,
            texture_path,
            Flip::None,
            &TextureSettings::new(),
        )?;
        let (width, height) = (
            texture.get_width() as Scalar,
            texture.get_height() as Scalar,
        );
        let mut atlas = vec![[0.0, 0.0, width, height]];
        let mut normalized_atlas = vec![[0.0, 0.0, 1.0, 1.0]];
        if let Some(atlas_path) = atlas_path {
            let parsed_atlas = Self::parse_atlas(atlas_path)?;
            if parsed_atlas.len() > 0 {
                atlas = parsed_atlas;
                normalized_atlas = Self::normalize_atlas(&atlas, width, height);
            }
        }
        Ok(TextureAtlas {
            texture: texture,
            atlas: atlas,
            normalized_atlas: normalized_atlas,
        })
    }

    #[inline]
    pub fn texture_view_sampler(&self) -> PistonPipelineSampler {
        (self.texture.view.clone(), self.texture.sampler.clone())
    }

    #[inline]
    pub fn texture_extents(&self, index: usize) -> [Scalar; 4] {
        self.atlas[index]
    }

    #[inline]
    pub fn texture_offsets(&self, index: usize) -> (Scalar, Scalar) {
        let extents = &self.atlas[index];
        (extents[2] / 2.0, extents[3] / 2.0)
    }

    #[inline]
    pub fn texture_uv_extents(&self, index: usize) -> (f32, f32, f32, f32) {
        let extents = &self.normalized_atlas[index];
        (extents[0], extents[1], extents[2], extents[3])
    }

    fn parse_atlas<P: AsRef<Path>>(path: P) -> Result<Vec<[Scalar; 4]>, Box<Error>> {
        let mut atlas = vec![];
        let file = File::open(path)?;
        for line in BufReader::new(file).lines() {
            let line = line?;
            let line = line.trim();
            if line.len() > 0 && !line.starts_with('#') {
                atlas.push(serde_json::from_str(line)?);
            }
        }
        Ok(atlas)
    }

    fn normalize_atlas(
        atlas: &[[Scalar; 4]],
        width: Scalar,
        height: Scalar,
    ) -> Vec<[f32; 4]> {
        atlas
            .iter()
            .map(|extents| {
                [
                    (extents[0] / width) as f32,
                    (extents[1] / height) as f32,
                    (extents[2] / width) as f32,
                    (extents[3] / height) as f32,
                ]
            }).collect()
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

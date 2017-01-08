//! Nature of code - Following the book... in Rust, with Piston!
//! http://natureofcode.com/
//!
//! Simple application framework, similar to the Processing environment
//! used in the book.

extern crate piston_window;
extern crate sdl2_window;

pub use piston_window::*;

use sdl2_window::Sdl2Window;

pub trait PistonApp {
    fn setup(&mut self, _context: Context, _gl: &mut G2d, _args: &RenderArgs) {}

    fn draw(&mut self, context: Context, gl: &mut G2d, args: &RenderArgs);

    fn run<T: Into<String>>(title: T, app: &mut Self) {
        let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(title, [640, 480])
            .exit_on_esc(true)
            .resizable(false)
            .vsync(true)
            .build()
            .unwrap();
        let mut first = true;
        while let Some(e) = window.next() {
            if let Some(args) = e.render_args() {
                window.draw_2d(&e, |context, gl| {
                    if first {
                        first = false;
                        app.setup(context, gl, &args);
                    }
                    app.draw(context, gl, &args);
                });
            }
        }
    }
}

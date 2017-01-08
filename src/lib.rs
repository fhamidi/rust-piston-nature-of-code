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
    fn setup<W, E>(&mut self, window: &mut PistonWindow<W>, e: &E, args: &RenderArgs)
        where E: GenericEvent,
              W: OpenGLWindow,
              W::Event: GenericEvent;

    fn draw<W, E>(&mut self, window: &mut PistonWindow<W>, e: &E, args: &RenderArgs)
        where E: GenericEvent,
              W: OpenGLWindow,
              W::Event: GenericEvent;

    fn run<T>(title: T, app: &mut Self)
        where T: Into<String>
    {
        let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(title, [640, 480])
            .exit_on_esc(true)
            .resizable(false)
            .vsync(true)
            .build()
            .unwrap();
        let mut first = true;
        while let Some(e) = window.next() {
            if let Some(args) = e.render_args() {
                if first {
                    first = false;
                    app.setup(&mut window, &e, &args);
                }
                app.draw(&mut window, &e, &args);
            }
        }
    }
}

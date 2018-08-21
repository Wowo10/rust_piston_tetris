extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod app;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let size: u32 = 20;
    let width: u8 = 10;
    let heigth: u8 = 30;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Tetris", [width as u32 * size, heigth as u32 * size])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Create a new game and run it.
    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        scene: Vec::new(),
        size: size,
    };

    app.init(width, heigth);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

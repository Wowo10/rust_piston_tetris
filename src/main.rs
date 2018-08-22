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

use app::timers::*;

fn main() {
    // Change this to OpenGL::V2_1 if not working.

    let start = Timer::create();

    let opengl = OpenGL::V3_2;

    let size: u32 = 20;
    let width: u8 = 10;
    let heigth: u8 = 25;

    let temp = WindowSettings::new("WowoTetris", [width as u32 * size, heigth as u32 * size])
        .opengl(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create an Glutin window.
    let mut window: Window = temp;

    // Create a new game and run it.
    let mut app = app::App::create(GlGraphics::new(opengl), size, width, heigth);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }

    let duration = start.get_elapsed();

    println!(
        "update: {}, render: {}, update/s:{}, render/s:{}, duration:{}ms",
        app.updateframes,
        app.renderframes,
        app.updateframes / ((duration / 1000) as usize),
        app.renderframes / ((duration / 1000) as usize),
        duration
    );
}

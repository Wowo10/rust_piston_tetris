extern crate find_folder;
extern crate piston_window;
extern crate rand;

use piston_window::*;

mod app;

use app::timers::*;

fn main() {
    let start = Timer::create();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let size: u32 = 20;
    let width: u8 = 10;
    let heigth: u8 = 25;

    let mut window: PistonWindow =
        WindowSettings::new("WowoTetris", [width as u32 * size, heigth as u32 * size])
            .opengl(opengl)
            .vsync(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut app = app::App::create(size, width, heigth);
    
    while let Some(e) = window.next() {
        match e {
            Input::Release(Button::Keyboard(key)) => {}

            Input::Update(args) => {
                app.update(args);
            }

            Input::Render(args) => {
                app.render(&mut window, e, args);
            }

            _ => {}
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

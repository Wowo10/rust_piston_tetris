extern crate find_folder;
extern crate piston_window;
extern crate rand;

use piston_window::*;

mod app;
mod config;

use app::timers::*;

fn main() {
    let start = Timer::create();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut config = config::Config::create("config.csv");

    let size: u32 = config.read("size").parse::<u32>().unwrap();
    let width: u8 = config.read("width").parse::<u8>().unwrap();
    let heigth: u8 = config.read("height").parse::<u8>().unwrap();

    let mut window: PistonWindow =
        WindowSettings::new("WowoTetris", [width as u32 * size, heigth as u32 * size])
            .opengl(opengl)
            .vsync(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut app = app::App::create(
        size,
        width,
        heigth,
        config.read_color("background_color"),
        config.read_color("border_color"),
        config.read_color("active_color"),
        config.read_color("taken_color"),
        config.read("offset").parse::<u8>().unwrap(),
    );

    while let Some(e) = window.next() {
        match e {
            Input::Release(Button::Keyboard(key)) => {
                app.handle_input(key);
            },

            Input::Update(args) => {
                app.update(args);
            },

            Input::Render(args) => {
                app.render(&mut window, e, args);
            },

            _ => {}
        }

        if app.exit {
            break;
        };
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

    println!("\nGame Over! Score: {}", app.score);
}

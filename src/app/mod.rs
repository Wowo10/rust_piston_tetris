use opengl_graphics::GlGraphics;
use piston::input::*;

mod state;
use self::state::State;

extern crate rand;
use app::rand::prelude::*;

pub mod timers;

mod block;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub scene: Vec<Vec<State>>,

    pub size: u32,
    pub renderframes: usize,
    pub updateframes: usize,
    pub timers: timers::Timers,

    pub activeblock: Vec<[u8; 2]>,
}

impl App {
    pub fn create(gl: GlGraphics, size: u32,width: u8, height: u8) -> Self {
        let mut temp = App {
            gl: gl,
            scene: Vec::new(),
            size: size,
            renderframes: 0,
            updateframes: 0,
            timers: timers::new_timers(),
            activeblock: Vec::new(),
        };

        temp.init(width, height);
        temp
    }

    fn init(&mut self, width: u8, height: u8) {
        for _ in 0..width {
            let mut v: Vec<State> = Vec::new();
            for _ in 0..height {
                v.push(State::Free);
            }
            &self.scene.push(v);
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.renderframes += 1;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let offset = 4; //should be even

        let square = rectangle::square(0.0, 0.0, self.size as f64);
        let squareinner = rectangle::square(
            (offset / 2) as f64,
            (offset / 2) as f64,
            (self.size - offset) as f64,
        );

        let size = (self.size as f64) / 2.0;

        let width = self.scene.len();
        let heigth = self.scene[0].len();

        let scene = &self.scene;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            for i in 0..width {
                for j in 0..heigth {
                    let transposition = c.transform
                        .trans(size * 2.0 * i as f64, size * 2.0 * j as f64);
                    rectangle(GREEN, square, transposition, gl);

                    let color = match &scene[i][j] {
                        State::Free => BLACK,
                        State::Taken => RED,
                        _ => BLUE,
                    };

                    rectangle(color, squareinner, transposition, gl);
                }
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        self.updateframes += 1;

        if self.timers.updatetimer.did_pass(2000) {
            println!("passed");

            self.timers.updatetimer.reset()
        }
    }

    fn random_pos_square(&mut self) {
        let mut rng = thread_rng();

        // let mut rand_x: usize = rng.gen(); //% &self.scene.len();
        // rand_x %= &self.scene.len();

        // let mut rand_y: usize = rng.gen(); //% &self.scene[0].len();
        // rand_y %= &self.scene[0].len();

        // let random: i8 = rng.gen(); // % 3;

        // self.scene[rand_x][rand_y] = match random % 3 {
        //     0 => State::Free,
        //     1 => State::Taken,
        //     _ => State::Active,
        // };
    }
}

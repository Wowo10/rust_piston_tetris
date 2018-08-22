extern crate find_folder;
extern crate piston_window;

use piston_window::*;

mod state;
use self::state::State;

extern crate rand;
use app::rand::prelude::*;

pub mod timers;

mod block;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const OFFSET: u32 = 4; //should be even

pub struct App {
    //pub gl: GlGraphics, // OpenGL drawing backend.
    pub scene: Vec<Vec<State>>,

    pub size: u32,
    pub renderframes: usize,
    pub updateframes: usize,
    pub timers: timers::Timers,

    pub activeblock: Vec<[u8; 2]>,
}

impl App {
    pub fn create(size: u32, width: u8, height: u8) -> Self {
        let mut temp = App {
            //gl: gl,
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

        self.activeblock.push([width / 2, 0]);
    }

    fn clear_board(&mut self) {
        for row in &mut self.scene {
            for state in row {
                match state {
                    State::Taken => {}
                    _ => *state = State::Free,
                };
            }
        }
    }

    pub fn render(&mut self, window: &mut PistonWindow, e: Input, args: RenderArgs) {
        //use graphics::*;

        self.renderframes += 1;

        let square = rectangle::square(0.0, 0.0, self.size as f64);
        let squareinner = rectangle::square(
            (OFFSET / 2) as f64,
            (OFFSET / 2) as f64,
            (self.size - OFFSET) as f64,
        );

        let size = (self.size as f64) / 2.0;

        let width = self.scene.len();
        let heigth = self.scene[0].len();

        let scene = &self.scene;

        window.draw_2d(&e, |c, g| {
            clear(GREEN, g);

            for i in 0..width {
                for j in 0..heigth {
                    let transposition = c.transform
                        .trans(size * 2.0 * i as f64, size * 2.0 * j as f64);
                    rectangle(GREEN, square, transposition, g);

                    let color = match &scene[i][j] {
                        State::Free => BLACK,
                        State::Taken => RED,
                        _ => BLUE,
                    };

                    rectangle(color, squareinner, transposition, g);
                }
            }
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.updateframes += 1;

        for block in &mut self.activeblock {
            self.scene[block[0] as usize][block[1] as usize] = State::Active;
        }

        if self.timers.updatetimer.did_pass(2000) {
            println!("passed");

            self.clear_board();

            let possible = true; //TODO
            if possible {
                for block in &mut self.activeblock {
                    block[1] += 1;
                }
            }

            self.timers.updatetimer.reset()
        }
    }

    fn random_pos_square(&mut self) {
        let mut rng = thread_rng();

    }
}

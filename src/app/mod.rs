extern crate find_folder;
extern crate piston_window;

use piston_window::*;

mod state;
use self::state::State;

extern crate rand;
//use app::rand::prelude::*;

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

        self.spawn_new_active_block(width / 2);
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

    pub fn render(&mut self, window: &mut PistonWindow, e: Input, _args: RenderArgs) {
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

    pub fn update(&mut self, _args: UpdateArgs) {
        self.updateframes += 1;

        self.clear_board();

        for block in &mut self.activeblock {
            self.scene[block[0] as usize][block[1] as usize] = State::Active;
        }

        if self.timers.updatetimer.did_pass(2000) {
            if !&self.move_down() { //keep in mind tha bug bro tomorrow
                while self.activeblock.len() != 0 {
                    if let Some(block) = self.activeblock.pop() {
                        self.scene[block[0] as usize][block[1] as usize] = State::Taken;
                    }
                }
                let startpos = (self.scene.len() / 2 as usize) as u8;
                self.spawn_new_active_block(startpos);
            }
        }
    }

    pub fn handle_input(&mut self, key: Key) {
        match key {
            Key::Left | Key::A => {
                &mut self.move_left();
            }
            Key::Right | Key::D => {
                &mut self.move_right();
            }
            Key::Down | Key::S => {
                &mut self.move_down();
            }
            Key::Up | Key::W => {
                &mut self.rotate();
            }
            Key::Space => {
                &mut self.drop();
            }

            _ => {}
        }
    }

    fn move_left(&mut self) {
        let possible = true; //TODO
        if possible {
            for block in &mut self.activeblock {
                block[0] -= 1;
            }
        }
    }

    fn move_right(&mut self) {
        let possible = true; //TODO
        if possible {
            for block in &mut self.activeblock {
                block[0] += 1;
            }
        }
    }

    fn move_down(&mut self) -> bool {
        let possible = true; //TODO

        let mut lowest = 0;

        if possible {
            for block in &mut self.activeblock {
                block[1] += 1;
                if block[1] > lowest {
                    lowest = block[1];
                }
            }
        }
        self.timers.updatetimer.reset();

        lowest != ((&self.scene[0].len() - 1) as u8) //temporary rly
    }

    fn rotate(&mut self) {
        println!("Rotated!"); //EZ when it is singe huh?
    }

    fn drop(&mut self) {
        //animate would be appreciated

        let mut able = true; //TODO
        while able {
            able = self.move_down();
        }
    }

    fn spawn_new_active_block(&mut self, startpos: u8) {
        self.activeblock.push([startpos, 0]); //TODO: add randoming actual blocks
    }
}

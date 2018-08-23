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

    pub exit: bool,
    pub score: u32,
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
            exit: false,
            score: 0,
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

        self.spawn_new_active_block();
    }

    fn clear_board(&mut self) {
        for row in &mut self.scene {
            for state in row {
                match *state {
                    State::Taken => {}
                    _ => *state = State::Free,
                }
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
            if self.able_move_down() {
                self.move_down();
            } else {
                self.spawn_new_active_block();
            }
        }
    }

    pub fn handle_input(&mut self, key: Key) {
        match key {
            Key::Left | Key::A => {
                &self.move_left();
            }
            Key::Right | Key::D => {
                &self.move_right();
            }
            Key::Down | Key::S => {
                &self.move_down();
            }
            Key::Up | Key::W => {
                &self.rotate();
            }
            Key::Space => {
                &self.drop();
            }

            _ => {}
        }
    }

    fn able_move_left(&self) -> bool {
        for block in &self.activeblock {
            let x_pos = block[0];
            if x_pos == 0 {
                return false;
            }
            match &self.scene[(x_pos - 1) as usize][block[1] as usize] {
                State::Taken => {
                    return false;
                }
                _ => {}
            }
        }
        true
    }

    fn move_left(&mut self) {
        let possible = self.able_move_left();
        if possible {
            for block in &mut self.activeblock {
                block[0] -= 1;
            }
        }
    }

    fn able_move_right(&self) -> bool {
        for block in &self.activeblock {
            let x_pos = block[0];
            if x_pos == (&self.scene.len() - 1) as u8 {
                return false;
            }
            match &self.scene[(x_pos + 1) as usize][block[1] as usize] {
                State::Taken => {
                    return false;
                }
                _ => {}
            }
        }
        true
    }

    fn move_right(&mut self) {
        let possible = self.able_move_right();
        if possible {
            for block in &mut self.activeblock {
                block[0] += 1;
            }
        }
    }

    fn able_move_down(&self) -> bool {
        for block in &self.activeblock {
            let y_pos = block[1];
            if y_pos == (&self.scene[0].len() - 1) as u8 {
                return false;
            }
            match &self.scene[block[0] as usize][(block[1] + 1) as usize] {
                State::Taken => {
                    return false;
                }
                _ => {}
            }
        }
        true
    }

    fn move_down(&mut self) {
        let possible = self.able_move_down(); //TODO

        if possible {
            for block in &mut self.activeblock {
                block[1] += 1;
            }
        }
        self.timers.updatetimer.reset();
    }

    fn rotate(&mut self) {
        println!("Rotated!"); //EZ when it is singe huh?
    }

    fn drop(&mut self) {
        //animate would be appreciated

        while self.able_move_down() {
            self.move_down();
        }

        self.spawn_new_active_block();
    }

    fn check_lines(&mut self) {
        for i in 0..self.scene[0].len() {
            let mut test = true;
            for j in 0..self.scene.len() {
                match self.scene[j][i] {
                    State::Taken => {}
                    _ => {
                        test = false;
                        break;
                    }
                }
            }

            if test {
                self.score += 20;
                for j in 0..self.scene.len() {
                    self.scene[j][i] = State::Free;
                }

                for k in (1..i + 1).rev() {
                    for j in 0..self.scene.len() {
                        self.scene[j][k] = self.scene[j][k - 1];
                    }
                }
            }
        }
    }

    fn spawn_new_active_block(&mut self) {
        while self.activeblock.len() != 0 {
            if let Some(block) = self.activeblock.pop() {
                self.scene[block[0] as usize][block[1] as usize] = State::Taken;
                if block[1] == 0 {
                    self.exit = true;
                    return;
                }
            }
        }

        self.check_lines();

        let startpos = (self.scene.len() / 2 as usize) as u8;

        for block in App::random_block(startpos) {
            self.activeblock.push(block);
        }
    }

    pub fn random_block(startpos: u8) -> Vec<[u8; 2]> {
        
        let mut rng = thread_rng();

        let vector: Vec<[u8; 2]>;
        for i in 0..4{ //kek tetris

        }
        
        vec![[startpos-1, 0], [startpos-2, 0]]
    }
}

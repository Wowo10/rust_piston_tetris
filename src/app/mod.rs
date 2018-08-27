extern crate find_folder;
extern crate piston_window;

use piston_window::*;

mod state;
use self::state::State;

extern crate rand;
use app::rand::prelude::*;

pub mod timers;

mod block;
mod constants;
use self::constants::Constants;

const OFFSET: u32 = 4; //should be even

pub struct App {
    pub scene: Vec<Vec<State>>,

    pub size: u32,
    pub renderframes: usize,
    pub updateframes: usize,
    pub timers: timers::Timers,

    pub activeblock: Vec<[u8; 2]>,

    pub exit: bool,
    pub score: u32,

    pub settings: Constants,
}

impl App {
    pub fn create(
        size: u32,
        width: u8,
        height: u8,
        color_background: [f32; 4],
        color_border: [f32; 4],
        color_active: [f32; 4],
        color_taken: [f32; 4],
        offset: u8,
    ) -> Self {
        let mut temp = App {
            scene: Vec::new(),
            size: size,
            renderframes: 0,
            updateframes: 0,
            timers: timers::new_timers(),
            activeblock: Vec::new(),
            exit: false,
            score: 0,

            settings: Constants {
                COLOR_BACKGROUND: color_background,
                COLOR_BORDER: color_border,
                COLOR_ACTIVE: color_active,
                COLOR_TAKEN: color_taken,
                OFFSET: offset,
            },
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
            (self.settings.OFFSET / 2) as f64,
            (self.settings.OFFSET / 2) as f64,
            (self.size - self.settings.OFFSET as u32) as f64,
        );

        let size = (self.size as f64) / 2.0;

        let width = self.scene.len();
        let heigth = self.scene[0].len();

        let scene = &self.scene;

        window.draw_2d(&e, |c, g| {
            clear(self.settings.COLOR_BORDER, g);

            for i in 0..width {
                for j in 0..heigth {
                    let transposition = c.transform
                        .trans(size * 2.0 * i as f64, size * 2.0 * j as f64);
                    rectangle(self.settings.COLOR_BORDER, square, transposition, g);

                    let color = match &scene[i][j] {
                        State::Free => self.settings.COLOR_BACKGROUND,
                        State::Taken => self.settings.COLOR_TAKEN,
                        _ => self.settings.COLOR_ACTIVE,
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
            Key::Up | Key::W => loop {
                match &self.rotate() {
                    block::RotateResult::DOWN | block::RotateResult::OK => {
                        break;
                    }
                    block::RotateResult::UP => {
                        self.move_down();
                    }
                    block::RotateResult::LEFT => {
                        self.move_right();
                    }
                    block::RotateResult::RIGHT => {
                        self.move_left();
                    }
                };
            },
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
        let possible = self.able_move_down();

        if possible {
            for block in &mut self.activeblock {
                block[1] += 1;
            }
        }
        self.timers.updatetimer.reset();
    }

    fn rotate(&mut self) -> block::RotateResult {
        let center = self.activeblock[1];
        self.activeblock.remove(1);

        let copy = self.activeblock.to_vec();

        self.activeblock.clear();

        for (i, &block) in copy.iter().enumerate() {
            let vec = App::substract_vectors(
                App::translate_to_signed_vector(center),
                App::translate_to_signed_vector(block),
            );

            let vec = App::rotate_vector_90(vec);

            let vec = App::add_vectors(App::translate_to_signed_vector(center), vec);

            if vec[0] < 0 {
                self.activeblock = copy.to_vec();
                self.activeblock.insert(1, center);
                return block::RotateResult::LEFT;
            } else if vec[1] < 0 {
                self.activeblock = copy.to_vec();
                self.activeblock.insert(1, center);
                return block::RotateResult::UP;
            } else if vec[0] >= self.scene.len() as i8 {
                self.activeblock = copy.to_vec();
                self.activeblock.insert(1, center);
                return block::RotateResult::RIGHT;
            } else if vec[1] >= self.scene[0].len() as i8 {
                self.activeblock = copy.to_vec();
                self.activeblock.insert(1, center);
                return block::RotateResult::DOWN;
            }

            self.activeblock
                .push(App::translate_to_unsigned_vector(vec));
            if i == 1 {
                //hack to keep second element the same
                self.activeblock.push(center);
            }
        }

        block::RotateResult::OK
    }

    fn translate_to_signed_vector(vector: [u8; 2]) -> [i8; 2] {
        [vector[0] as i8, vector[1] as i8]
    }

    fn translate_to_unsigned_vector(vector: [i8; 2]) -> [u8; 2] {
        [vector[0] as u8, vector[1] as u8]
    }

    fn add_vectors(vector1: [i8; 2], vector2: [i8; 2]) -> [i8; 2] {
        [vector1[0] + vector2[0], vector1[1] + vector2[1]]
    }

    fn substract_vectors(vector1: [i8; 2], vector2: [i8; 2]) -> [i8; 2] {
        [vector1[0] - vector2[0], vector1[1] - vector2[1]]
    }

    fn rotate_vector_90(vector: [i8; 2]) -> [i8; 2] {
        [vector[1] * -1, vector[0]]
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

        let startpos = (self.scene.len() / 2 as usize) as u8 - 1;

        for block in App::random_hardcoded_block(startpos) {
            self.activeblock.push(block);
        }
    }

    pub fn random_hardcoded_block(startpos: u8) -> Vec<[u8; 2]> {
        let mut pos: [u8; 2] = [startpos, 0];
        let mut vector = vec![[pos[0], pos[1]]];

        let mut rng = thread_rng();

        let random: u8 = rng.gen();

        match random % 7 {
            0 => {
                //line
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
            }
            1 => {
                //2x2 block
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
                pos[0] += 1;
                pos[1] -= 1;
                vector.push([pos[0], pos[1]]);
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
            }
            2 => {
                //z
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
                pos[0] += 1;
                vector.push([pos[0], pos[1]]);
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
            }
            3 => {
                //s
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
                pos[0] -= 1;
                vector.push([pos[0], pos[1]]);
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
            }
            4 => {
                //L
                pos[0] += 1;
                vector.push([pos[0], pos[1]]);
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
            }
            5 => {
                //~L
                pos[0] -= 1;
                vector.push([pos[0], pos[1]]);
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
            }
            6 => {
                //T
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
                pos[1] += 1;
                vector.push([pos[0], pos[1]]);
                pos[0] += 1;
                pos[1] -= 1;
                vector.push([pos[0], pos[1]]);
            }
            _ => {}
        }

        vector
    }
}

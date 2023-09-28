pub mod constants;
pub mod init;
pub mod tetromino;
pub mod well;

use term2d::model::rgba::Rgba;
use term2d::App;

use self::constants::{WELL_TOP, WELL_LEFT};
use self::tetromino::{Tetromino, TetrominoKind};
use self::well::Well;

// numbers from glibc, https://en.wikipedia.org/wiki/Linear_congruential_generator
const M: u64 = 2147483648;
const A: u64 = 1103515245;
const C: u64 = 12345;

pub struct Model {
    pub debug: i32,
    pub game_over: bool,
    pub gravity: u64,
    pub help: bool,
    pub pause: bool,
    pub random: u64,
    pub score: u32,
    pub tetromino: Tetromino,
    pub well: Well,
}

impl Model {
    pub fn new(_app: &App) -> Self {
        let well = Well::new(10, 20);
        //let well = Well::new_debug();

        Self {
            debug: 0,
            game_over: false,
            gravity: 10,
            help: false,
            pause: false,
            random: 7,
            score: 0,
            tetromino: Tetromino::new(TetrominoKind::I, well.clone()),
            well,
        }
    }

    pub fn reset(&mut self, app: &App) {
        *self = Self::new(app);
    }

    pub fn is_paused(&self) -> bool {
        self.game_over || self.help || self.pause
    }

    pub fn move_tetromino_down(&mut self) {
        if self.tetromino.move_down() {
            // TODO: clean code

            // place tetromino in well
            let ps = self.tetromino.get_tetromino_points();
            for p in ps {
                let i = (self.well.width * (p.y - WELL_TOP) + (p.x - WELL_LEFT)) as usize;
                self.well.colors[i] = Some(Rgba::red());
            }

            // delete full rows
            self.score += self.well.delete_full_rows();

            // update gravity
            self.gravity = (10 - self.score as i32 / 10).max(1) as u64;

            // set new tetromino
            self.random = (A * self.random + C) % M;
            self.tetromino = Tetromino::new(TetrominoKind::from(self.random), self.well.clone());
        }
    }
}

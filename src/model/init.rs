use term2d::App;

use super::tetromino::Tetromino;
use super::tetromino::TetrominoKind;
use super::well::Well;

pub struct Model {
    pub debug: i32,
    pub gravity: u64,
    pub random: u64,
    pub score: u32,
    pub tetromino: Tetromino,
    pub well: Well,
}

impl Model {
    pub fn reset(&mut self, app: &App) {
        let m = init_model(app);
        self.debug = m.debug;
        self.gravity = m.gravity;
        self.random = m.random;
        self.score = m.score;
        self.tetromino = m.tetromino;
        self.well = m.well;
    }
}

pub fn init_model(_app: &App) -> Model {
    let well = Well::new(10, 20);
    //let well = Well::new_debug();

    Model {
        debug: 0,
        gravity: 10,
        random: 7,
        score: 0,
        tetromino: Tetromino::new(TetrominoKind::I, well.clone()),
        well,
    }
}

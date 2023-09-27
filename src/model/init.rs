use term2d::App;

use super::tetromino::Tetromino;
use super::tetromino::TetrominoKind;
use super::well::Well;

pub struct Model {
    pub debug: i32,
    pub gravity: u64,
    pub random: u64,
    pub tetromino: Tetromino,
    pub well: Well,
}

pub fn init_model(_app: &App) -> Model {
    let well = Well::new(10, 20);
    //let well = Well::new_debug();

    Model {
        debug: 0,
        gravity: 10,
        random: 7,
        tetromino: Tetromino::new(TetrominoKind::I, well.clone()),
        well,
    }
}

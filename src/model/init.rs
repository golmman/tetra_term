use term2d::App;

use super::tetromino::{Tetromino, Tetromino2, TetrominoKind};
use super::tetromino_i::TetrominoI;
use super::well::Well;

pub struct Model {
    pub debug: i32,
    pub tetromino: Tetromino2,
    pub well: Well,
}

pub fn init_model(_app: &App) -> Model {
    //let well = Well::new(10, 20);
    let well = Well::new_debug();

    Model {
        debug: 0,
        tetromino: Tetromino2::new(TetrominoKind::I, well.clone()),
        well,
    }
}

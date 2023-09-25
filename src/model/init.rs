use term2d::App;

use super::tetromino::Tetromino;
use super::tetromino_i::TetrominoI;
use super::well::Well;

pub struct Model {
    pub debug: i32,
    pub tetromino: Box<dyn Tetromino>,
    pub well: Well,
}

pub fn init_model(_app: &App) -> Model {
    //let well = Well::new(10, 20);
    let well = Well::new_debug();

    Model {
        debug: 0,
        tetromino: Box::new(TetrominoI::new(well.clone())),
        well,
    }
}

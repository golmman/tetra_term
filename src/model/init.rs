use term2d::model::point::Point;
use term2d::App;

use super::tetromino::Tetromino;
use super::tetromino_i::TetrominoI;

pub struct Model {
    pub debug: i32,
    pub tetromino: Box<dyn Tetromino>,
}

pub fn init_model(_app: &App) -> Model {
    Model {
        debug: 0,
        tetromino: Box::new(TetrominoI::new()),
    }
}

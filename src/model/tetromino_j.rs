use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::Canvas;

use super::tetromino::Tetromino;

pub struct TetrominoJ {
    position: Point,
}

impl TetrominoJ {
    pub fn new() -> Self {
        Self {
            position: Point::new(0, 0),
        }
    }
}

impl Tetromino for TetrominoJ {
    fn draw(&self, canvas: &mut term2d::view::canvas::halfblock::HalfblockCanvas) {
        canvas.draw_pixel(&Point::new(51, 7), &Rgba::blue());
    }

    fn drop(&mut self) {
        todo!()
    }

    fn is_falling(&self) -> bool {
        todo!()
    }

    fn move_left(&mut self) {
        todo!()
    }

    fn move_right(&mut self) {
        todo!()
    }

    fn move_down(&mut self) {
        todo!()
    }

    fn rotate(&mut self) {
        todo!()
    }
}

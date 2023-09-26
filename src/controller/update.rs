use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::App;

use crate::model::init::Model;
use crate::model::tetromino_i::TetrominoI;
use crate::model::tetromino_j::TetrominoJ;

pub fn update_model(_app: &App, model: &mut Model, event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            Key::Ctrl('c') => return false,

            Key::Char('s') | Key::Char('j') | Key::Down => model.tetromino.move_down(),
            Key::Char('a') | Key::Char('h') | Key::Left => model.tetromino.move_left(),
            Key::Char('d') | Key::Char('l') | Key::Right => model.tetromino.move_right(),

            Key::Char('w') => {
                if model.debug % 2 == 0 {
                    model.tetromino = Box::new(TetrominoJ::new(model.well.clone()));
                } else {
                    model.tetromino = Box::new(TetrominoI::new(model.well.clone()));
                }
                model.debug += 1;
            }
            Key::Char('r') => {
                model.tetromino.rotate();
            }
            _ => {}
        },
        Event::Resize(_) => {}
        Event::Elapse => {}
    }

    true
}

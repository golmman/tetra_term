use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::App;

use crate::model::Model;
use crate::model::tetromino::Tetromino;
use crate::model::tetromino::TetrominoKind;

pub fn update_model(app: &App, model: &mut Model, event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            Key::Ctrl('c') => return false,

            Key::Char('w') | Key::Char('k') | Key::Up => model.rotate_tetromino(),
            Key::Char('a') | Key::Char('h') | Key::Left => model.move_tetromino_left(),
            Key::Char('s') | Key::Char('j') | Key::Down => model.move_tetromino_down(),
            Key::Char('d') | Key::Char('l') | Key::Right => model.move_tetromino_right(),

            Key::Char(' ') => model.drop_tetromino(),

            Key::Char('r') => model.reset(app),
            Key::Char('1') => model.toggle_help(),
            Key::Char('p') => model.toggle_pause(),

            Key::Char('o') => {
                model.debug += 1;
                match model.debug % 7 {
                    0 => model.tetromino = Tetromino::new(TetrominoKind::I, model.well.clone()),
                    1 => model.tetromino = Tetromino::new(TetrominoKind::J, model.well.clone()),
                    2 => model.tetromino = Tetromino::new(TetrominoKind::L, model.well.clone()),
                    3 => model.tetromino = Tetromino::new(TetrominoKind::O, model.well.clone()),
                    4 => model.tetromino = Tetromino::new(TetrominoKind::S, model.well.clone()),
                    5 => model.tetromino = Tetromino::new(TetrominoKind::T, model.well.clone()),
                    6 => model.tetromino = Tetromino::new(TetrominoKind::Z, model.well.clone()),
                    _ => panic!(),
                }
            }
            _ => {}
        },
        Event::Resize(_) => {}
        Event::Elapse => {
            if app.frame_count % model.gravity == 0 {
                model.move_tetromino_down();
            }
        }
    };

    true
}

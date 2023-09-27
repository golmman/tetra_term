use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::model::rgba::Rgba;
use term2d::App;

use crate::model::constants::WELL_LEFT;
use crate::model::constants::WELL_TOP;
use crate::model::init::Model;
use crate::model::tetromino::Tetromino;
use crate::model::tetromino::TetrominoKind;

// numbers from glibc, https://en.wikipedia.org/wiki/Linear_congruential_generator
const M: u64 = 2147483648;
const A: u64 = 1103515245;
const C: u64 = 12345;

pub fn update_model(_app: &App, model: &mut Model, event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            Key::Ctrl('c') => return false,

            Key::Char('s') | Key::Char('j') | Key::Down => {
                if model.tetromino.move_down() {
                    let ps = model.tetromino.get_tetromino_points();
                    for p in ps {
                        let i = (model.well.width * (p.y - WELL_TOP) + (p.x - WELL_LEFT)) as usize;
                        model.well.colors[i] = Some(Rgba::red());
                    }

                    model.well.delete_full_rows();

                    model.random = (A * model.random + C) % M;
                    model.tetromino =
                        Tetromino::new(TetrominoKind::from(model.random), model.well.clone());
                }
            }
            Key::Char('a') | Key::Char('h') | Key::Left => model.tetromino.move_left(),
            Key::Char('d') | Key::Char('l') | Key::Right => model.tetromino.move_right(),

            Key::Char('w') => {
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
            Key::Char('r') => {
                model.tetromino.rotate();
            }
            _ => {}
        },
        Event::Resize(_) => {}
        Event::Elapse => {}
    };

    true
}

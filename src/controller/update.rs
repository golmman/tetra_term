use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::App;

use crate::model::Model;

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

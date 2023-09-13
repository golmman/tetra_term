use term2d::model::event::Event;
use term2d::model::key::Key;
use term2d::App;

use crate::model::init::Model;

pub fn update_model(app: &App, model: &mut Model, event: Event) -> bool {
    match event {
        Event::Key(key) => match key {
            Key::Char('q') => return false,
            Key::Ctrl('c') => return false,
            _ => {}
        },
        Event::Resize(_) => {}
        Event::Elapse => {
            model.pixel_point.x = 12 + (10.0 * (app.frame_count as f32 / 10.0).cos()) as i32;
            model.pixel_point.y = 12 + (10.0 * (app.frame_count as f32 / 10.0).sin()) as i32;
        }
    }

    true
}

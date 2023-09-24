use term2d::model::color::Color;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::App;

use crate::model::constants::WELL_HEIGHT;
use crate::model::constants::WELL_WIDTH;
use crate::model::init::Model;

pub fn draw_model(app: &App, model: &Model, canvas: &mut HalfblockCanvas) {
    canvas.clear();

    draw_well(model, canvas);
    draw_tetromino(model, canvas);

    //canvas.draw_text(
    //    &Point::new(0, 0),
    //    &Color::text(),
    //    &format!("press 'q' to quit, frame: {}", app.frame_count),
    //);
    canvas.display();
}

fn draw_tetromino(model: &Model, canvas: &mut HalfblockCanvas) {
    model.tetromino.draw(canvas);
}

fn draw_well(model: &Model, canvas: &mut HalfblockCanvas) {
    // see code page 437 for character codes

    let w = WELL_WIDTH;
    let h = WELL_HEIGHT;
    let color_text = &Color::text();

    for x in 1..w + 6 {
        canvas.draw_char(&Point::new(x, 0), color_text, '\u{2500}');
        canvas.draw_char(&Point::new(x, h + 3), color_text, '\u{2500}');
    }

    for y in 1..h / 2 + 1 {
        canvas.draw_char(&Point::new(0, y * 2), color_text, '\u{2502}');
        canvas.draw_char(&Point::new(w + 1, y * 2), color_text, '\u{2502}');
        canvas.draw_char(&Point::new(w + 6, y * 2), color_text, '\u{2502}');
    }

    canvas.draw_char(&Point::new(0, 0), color_text, '\u{250C}');
    canvas.draw_char(&Point::new(w + 6, 0), color_text, '\u{2510}');

    canvas.draw_char(&Point::new(w + 1, 0), color_text, '\u{252C}');
    canvas.draw_char(&Point::new(w + 1, h + 3), color_text, '\u{2534}');

    canvas.draw_char(&Point::new(0, h + 3), color_text, '\u{2514}');
    canvas.draw_char(&Point::new(w + 6, h + 3), color_text, '\u{2518}');
}

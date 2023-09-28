use term2d::model::color::Color;
use term2d::model::point::Point;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::App;

use crate::model::constants::WELL_LEFT;
use crate::model::constants::WELL_TOP;
use crate::model::Model;

pub fn draw_model(_app: &App, model: &Model, canvas: &mut HalfblockCanvas) {
    canvas.clear();
    draw_well(model, canvas);
    draw_tetromino(model, canvas);
    draw_score(model, canvas);
    draw_info(model, canvas);
    draw_help(model, canvas);
    draw_pause(model, canvas);
    draw_game_over(model, canvas);
    draw_frame(model, canvas);
    canvas.display();
}

fn draw_game_over(model: &Model, canvas: &mut HalfblockCanvas) {
    if !model.game_over {
        return;
    }

    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 8),
        &Color::text(),
        "GAME OVER",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 10),
        &Color::text(),
        "press r to",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 12),
        &Color::text(),
        "  reset",
    );
}

fn draw_pause(model: &Model, canvas: &mut HalfblockCanvas) {
    if !model.pause {
        return;
    }

    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP),
        &Color::text(),
        "  PAUSE",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 2),
        &Color::text(),
        "press p to",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 4),
        &Color::text(),
        " continue",
    );
}

fn draw_help(model: &Model, canvas: &mut HalfblockCanvas) {
    if !model.help {
        return;
    }

    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP),
        &Color::text(),
        "tetra_term",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 2),
        &Color::text(),
        "----------",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 4),
        &Color::text(),
        "q - quit",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 6),
        &Color::text(),
        "w - rotate",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 8),
        &Color::text(),
        "a - left",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 10),
        &Color::text(),
        "s - down",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 12),
        &Color::text(),
        "d - right",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 14),
        &Color::text(),
        "_ - drop",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 16),
        &Color::text(),
        "r - reset",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 18),
        &Color::text(),
        "p - pause",
    );
}

fn draw_info(model: &Model, canvas: &mut HalfblockCanvas) {
    canvas.draw_text(
        &Point::new(
            WELL_LEFT + model.well.width + 1,
            WELL_TOP + model.well.height - 6,
        ),
        &Color::text(),
        "PRESS",
    );
    canvas.draw_text(
        &Point::new(
            WELL_LEFT + model.well.width + 1,
            WELL_TOP + model.well.height - 4,
        ),
        &Color::text(),
        "1 FOR",
    );
    canvas.draw_text(
        &Point::new(
            WELL_LEFT + model.well.width + 1,
            WELL_TOP + model.well.height - 2,
        ),
        &Color::text(),
        "HELP",
    );
}

fn draw_score(model: &Model, canvas: &mut HalfblockCanvas) {
    canvas.draw_text(
        &Point::new(WELL_LEFT + model.well.width + 1, WELL_TOP + 9),
        &Color::text(),
        "SCORE",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT + model.well.width + 1, WELL_TOP + 10),
        &Color::text(),
        &format!("{:0>5}", model.score),
    );
}

fn draw_tetromino(model: &Model, canvas: &mut HalfblockCanvas) {
    model.tetromino.draw(canvas);
}

fn draw_frame(model: &Model, canvas: &mut HalfblockCanvas) {
    // see code page 437 for character codes

    let w = model.well.width;
    let h = model.well.height;
    let color_text = &Color::text();

    for x in 1..w + 7 {
        canvas.draw_char(&Point::new(x, 0), color_text, '\u{2500}');
        canvas.draw_char(&Point::new(x, h + 3), color_text, '\u{2500}');
    }

    for y in 1..h / 2 + 1 {
        canvas.draw_char(&Point::new(0, y * 2), color_text, '\u{2502}');
        canvas.draw_char(&Point::new(w + 1, y * 2), color_text, '\u{2502}');
        canvas.draw_char(&Point::new(w + 7, y * 2), color_text, '\u{2502}');
    }

    canvas.draw_char(&Point::new(0, 0), color_text, '\u{250C}');
    canvas.draw_char(&Point::new(w + 7, 0), color_text, '\u{2510}');

    canvas.draw_char(&Point::new(w + 1, 0), color_text, '\u{252C}');
    canvas.draw_char(&Point::new(w + 1, h + 3), color_text, '\u{2534}');

    canvas.draw_char(&Point::new(0, h + 3), color_text, '\u{2514}');
    canvas.draw_char(&Point::new(w + 7, h + 3), color_text, '\u{2518}');
}

fn draw_well(model: &Model, canvas: &mut HalfblockCanvas) {
    for y in 0..model.well.height {
        for x in 0..model.well.width {
            let i = (model.well.width * y + x) as usize;
            if let Some(color) = &model.well.colors[i] {
                canvas.draw_pixel(&Point::new(x + WELL_LEFT, y + WELL_TOP), color);
            }
        }
    }
}

use term2d::model::point::Point;
use term2d::model::rect::Rect;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::App;

use crate::model::constants::COLOR_TEXT;
use crate::model::constants::INFO_WIDTH;
use crate::model::constants::RGBA_BACKGROUND;
use crate::model::constants::WELL_LEFT;
use crate::model::constants::WELL_TOP;
use crate::model::Model;

pub fn draw_model(app: &App, model: &Model, canvas: &mut HalfblockCanvas) {
    clear_canvas(app.frame_count, model, canvas);

    draw_background(model, canvas);
    draw_well_background(model, canvas);
    draw_tetromino(model, canvas);
    draw_tetromino_next(model, canvas);
    draw_score(model, canvas);
    draw_info(model, canvas);
    draw_help(model, canvas);
    draw_pause(model, canvas);
    draw_game_over(model, canvas);
    draw_frame(model, canvas);

    canvas.display();
}

fn clear_canvas(frame_count: u64, model: &Model, canvas: &mut HalfblockCanvas) {
    if frame_count == model.clear_at_frame_count {
        canvas.clear();
    }
}

fn draw_tetromino_next(model: &Model, canvas: &mut HalfblockCanvas) {
    canvas.draw_text(
        &Point::new(WELL_LEFT + model.well.width + 1, WELL_TOP),
        &COLOR_TEXT,
        "NEXT",
    );

    let left = WELL_LEFT + model.well.width + 3;
    let top = WELL_TOP + 4;
    model.tetromino_next.draw_at(canvas, &Point::new(left, top));
}

fn draw_background(model: &Model, canvas: &mut HalfblockCanvas) {
    canvas.draw_rect_fill(
        &Rect {
            pos: Point::new(0, 0),
            size: Point::new(model.well.width + INFO_WIDTH + 2, model.well.height + 2),
        },
        &RGBA_BACKGROUND,
    );
}

fn draw_game_over(model: &Model, canvas: &mut HalfblockCanvas) {
    if !model.game_over {
        return;
    }

    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 8),
        &COLOR_TEXT,
        "GAME OVER",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 10),
        &COLOR_TEXT,
        "press r to",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 12),
        &COLOR_TEXT,
        "  reset",
    );
}

fn draw_pause(model: &Model, canvas: &mut HalfblockCanvas) {
    if !model.pause {
        return;
    }

    canvas.draw_text(&Point::new(WELL_LEFT, WELL_TOP), &COLOR_TEXT, "  PAUSE");
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 2),
        &COLOR_TEXT,
        "press p to",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 4),
        &COLOR_TEXT,
        " continue",
    );
}

fn draw_help(model: &Model, canvas: &mut HalfblockCanvas) {
    if !model.help {
        return;
    }

    canvas.draw_text(&Point::new(WELL_LEFT, WELL_TOP), &COLOR_TEXT, "tetra_term");
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 2),
        &COLOR_TEXT,
        "----------",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 4),
        &COLOR_TEXT,
        "q - quit",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 6),
        &COLOR_TEXT,
        "w - rotate",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 8),
        &COLOR_TEXT,
        "a - left",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 10),
        &COLOR_TEXT,
        "s - down",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 12),
        &COLOR_TEXT,
        "d - right",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 14),
        &COLOR_TEXT,
        "_ - drop",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 16),
        &COLOR_TEXT,
        "r - reset",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT, WELL_TOP + 18),
        &COLOR_TEXT,
        "p - pause",
    );
}

fn draw_info(model: &Model, canvas: &mut HalfblockCanvas) {
    canvas.draw_text(
        &Point::new(
            WELL_LEFT + model.well.width + 1,
            WELL_TOP + model.well.height - 4,
        ),
        &COLOR_TEXT,
        "1 FOR",
    );
    canvas.draw_text(
        &Point::new(
            WELL_LEFT + model.well.width + 1,
            WELL_TOP + model.well.height - 2,
        ),
        &COLOR_TEXT,
        "HELP",
    );
}

fn draw_score(model: &Model, canvas: &mut HalfblockCanvas) {
    canvas.draw_text(
        &Point::new(WELL_LEFT + model.well.width + 1, WELL_TOP + 10),
        &COLOR_TEXT,
        "SCORE",
    );
    canvas.draw_text(
        &Point::new(WELL_LEFT + model.well.width + 1, WELL_TOP + 12),
        &COLOR_TEXT,
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
    let color_text = &COLOR_TEXT;

    // top and bottom horizontal line
    for x in WELL_LEFT..w + INFO_WIDTH + WELL_LEFT + 1 {
        canvas.draw_char(&Point::new(x, 0), color_text, '\u{2500}');
        canvas.draw_char(&Point::new(x, h + 3), color_text, '\u{2500}');
    }

    // info area horizontal lines
    for x in WELL_LEFT + w..w + INFO_WIDTH + WELL_LEFT + 1 {
        canvas.draw_char(&Point::new(x, 10), color_text, '\u{2500}');
        canvas.draw_char(&Point::new(x, 16), color_text, '\u{2500}');
    }

    // left, mid and right vertical line
    for y in WELL_TOP / 2..h / 2 + 1 {
        canvas.draw_char(&Point::new(0, y * 2), color_text, '\u{2502}');
        canvas.draw_char(&Point::new(w + 1, y * 2), color_text, '\u{2502}');
        canvas.draw_char(
            &Point::new(w + INFO_WIDTH + 2, y * 2),
            color_text,
            '\u{2502}',
        );
    }

    // top left and right corners
    canvas.draw_char(&Point::new(0, 0), color_text, '\u{250C}');
    canvas.draw_char(&Point::new(w + INFO_WIDTH + 2, 0), color_text, '\u{2510}');

    // mid T-corners
    canvas.draw_char(&Point::new(w + 1, 0), color_text, '\u{252C}');
    canvas.draw_char(&Point::new(w + 1, h + 3), color_text, '\u{2534}');

    // bottom left and right corners
    canvas.draw_char(&Point::new(0, h + 3), color_text, '\u{2514}');
    canvas.draw_char(
        &Point::new(w + INFO_WIDTH + 2, h + 3),
        color_text,
        '\u{2518}',
    );
}

fn draw_well_background(model: &Model, canvas: &mut HalfblockCanvas) {
    for y in 0..model.well.height {
        for x in 0..model.well.width {
            let i = (model.well.width * y + x) as usize;
            if let Some(color) = &model.well.colors[i] {
                canvas.draw_pixel(&Point::new(x + WELL_LEFT, y + WELL_TOP), color);
            }
        }
    }
}

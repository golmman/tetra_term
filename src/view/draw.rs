use term2d::model::color::Color;
use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;
use term2d::App;

use crate::model::init::Model;

pub fn draw_model(app: &App, model: &Model, canvas: &mut HalfblockCanvas) {
    canvas.clear();
    canvas.draw_pixel(&model.pixel_point, &Rgba::red());
    canvas.draw_text(
        &Point::new(0, 0),
        &Color::text(),
        &format!("press 'q' to quit, frame: {}", app.frame_count),
    );
    canvas.display();
}

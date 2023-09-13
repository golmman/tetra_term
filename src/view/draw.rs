use term2d::{
    model::{color::Color, point::Point, rgba::Rgba},
    view::canvas::{halfblock::HalfblockCanvas, Canvas},
    App,
};

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

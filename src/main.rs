use controller::update::update_model;
use model::init::init_model;
use term2d::AppBuilder;
use view::draw::draw_model;

pub mod controller;
pub mod model;
pub mod view;

fn main() {
    AppBuilder::new(init_model)
        .event(update_model)
        .view(draw_model)
        .fps(10)
        .run();
}

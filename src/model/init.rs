use term2d::{model::point::Point, App};

pub struct Model {
    pub pixel_point: Point,
}

pub fn init_model(_app: &App) -> Model {
    Model {
        pixel_point: Point::new(0, 0),
    }
}

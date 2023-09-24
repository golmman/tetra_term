use term2d::model::color::Color;

#[derive(Clone)]
pub struct Well {
    pub colors: Vec<Option<Color>>,
    pub width: i32,
    pub height: i32,
}

impl Well {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            colors: vec![None; (width * height) as usize],
            width,
            height,
        }
    }
}

use term2d::model::rgba::Rgba;

#[derive(Clone)]
pub struct Well {
    pub colors: Vec<Option<Rgba>>,
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

    pub fn new_debug() -> Self {
        let colors = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 1, 1, 1, 1, //
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
            1, 0, 0, 0, 0, 0, 0, 0, 0, 1, //
            1, 0, 0, 0, 0, 0, 0, 0, 0, 1, //
            1, 0, 0, 0, 0, 0, 0, 0, 0, 1, //
            1, 0, 0, 0, 0, 0, 0, 0, 0, 1, //
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1, //
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1, //
            1, 1, 1, 0, 0, 0, 0, 1, 1, 1, //
            1, 1, 1, 0, 0, 0, 0, 1, 1, 1, //
            1, 1, 1, 1, 0, 0, 1, 1, 1, 1, //
            1, 1, 1, 1, 0, 0, 1, 1, 1, 1, //
            1, 1, 1, 1, 0, 0, 1, 1, 1, 1, //
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, //
        ];

        let colors = colors
            .into_iter()
            .map(|x| if x == 1 { Some(Rgba::red()) } else { None })
            .collect();
        Self {
            colors,
            width: 10,
            height: 20,
        }
    }
}

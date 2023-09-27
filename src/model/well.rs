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

    pub fn delete_full_rows(&mut self) -> u32 {
        let mut rows_deleted = 0;
        let mut new_colors: Vec<Option<Rgba>> = Vec::new();
        let empty_row: Vec<Option<Rgba>> = vec![None; self.width as usize];

        for y in 0..self.height {
            let mut row = Vec::new();
            let mut is_full = true;

            for x in 0..self.width {
                let i = (self.width * y + x) as usize;
                row.push(self.colors[i].clone());
                if self.colors[i].is_none() {
                    is_full = false;
                }
            }

            if is_full {
                let mut empty = empty_row.clone();
                empty.append(&mut new_colors);
                new_colors = empty;
                rows_deleted += 1;
            } else {
                new_colors.append(&mut row);
            }
        }

        self.colors = new_colors;
        rows_deleted
    }
}

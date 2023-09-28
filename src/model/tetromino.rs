use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::halfblock::HalfblockCanvas;
use term2d::view::canvas::Canvas;

use super::constants::WELL_LEFT;
use super::constants::WELL_TOP;
use super::random::Random;
use super::well::Well;

const MIC: u8 = 64;

#[derive(Clone, Copy)]
pub enum TetrominoKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl From<u32> for TetrominoKind {
    fn from(value: u32) -> Self {
        match value % 7 {
            0 => TetrominoKind::I,
            1 => TetrominoKind::J,
            2 => TetrominoKind::L,
            3 => TetrominoKind::O,
            4 => TetrominoKind::S,
            5 => TetrominoKind::T,
            6 => TetrominoKind::Z,
            _ => panic!(),
        }
    }
}

#[rustfmt::skip]
impl From<TetrominoKind> for Rgba {
    fn from(kind: TetrominoKind) -> Self {
        match kind {
            TetrominoKind::I => Rgba { r: MIC, g: 255, b: 255, a: 255 },
            TetrominoKind::J => Rgba { r: 127, g: 127, b: 255, a: 255 },
            TetrominoKind::L => Rgba { r: 255, g: 127, b: MIC, a: 255 },
            TetrominoKind::O => Rgba { r: 255, g: 255, b: MIC, a: 255 },
            TetrominoKind::S => Rgba { r: 127, g: 255, b: 127, a: 255 },
            TetrominoKind::T => Rgba { r: 255, g: MIC, b: 127, a: 255 },
            TetrominoKind::Z => Rgba { r: 255, g: 127, b: 127, a: 255 },
        }
    }
}

pub struct Tetromino {
    pub colors: [Rgba; 4],
    kind: TetrominoKind,
    position: Point,
    rotation: u8,
    well: Well,
}

impl Tetromino {
    pub fn new(random: &mut Random, well: Well) -> Self {
        let kind = TetrominoKind::from(random.next());
        let colors = Self::get_colors(kind, random);
        Self {
            colors,
            kind,
            position: Point::new(WELL_LEFT + 4, WELL_TOP - 1),
            rotation: 0,
            well,
        }
    }

    fn get_colors(kind: TetrominoKind, random: &mut Random) -> [Rgba; 4] {
        let c = Rgba::from(kind);

        let c0 = Rgba {
            r: (c.r as u32 - random.next() % MIC as u32) as u8,
            g: (c.g as u32 - random.next() % MIC as u32) as u8,
            b: (c.b as u32 - random.next() % MIC as u32) as u8,
            a: 255,
        };
        let c1 = Rgba {
            r: (c.r as u32 - random.next() % MIC as u32) as u8,
            g: (c.g as u32 - random.next() % MIC as u32) as u8,
            b: (c.b as u32 - random.next() % MIC as u32) as u8,
            a: 255,
        };
        let c2 = Rgba {
            r: (c.r as u32 - random.next() % MIC as u32) as u8,
            g: (c.g as u32 - random.next() % MIC as u32) as u8,
            b: (c.b as u32 - random.next() % MIC as u32) as u8,
            a: 255,
        };
        let c3 = Rgba {
            r: (c.r as u32 - random.next() % MIC as u32) as u8,
            g: (c.g as u32 - random.next() % MIC as u32) as u8,
            b: (c.b as u32 - random.next() % MIC as u32) as u8,
            a: 255,
        };
        [c0, c1, c2, c3]
    }

    pub fn get_tetromino_points(&self) -> [Point; 4] {
        match self.kind {
            TetrominoKind::I => self.get_i_points(),
            TetrominoKind::J => self.get_j_points(),
            TetrominoKind::L => self.get_l_points(),
            TetrominoKind::O => self.get_o_points(),
            TetrominoKind::S => self.get_s_points(),
            TetrominoKind::T => self.get_t_points(),
            TetrominoKind::Z => self.get_z_points(),
        }
    }

    fn get_i_points(&self) -> [Point; 4] {
        match self.rotation {
            0 | 2 => {
                let a = self.position.up();
                let b = a.down();
                let c = b.down();
                let d = c.down();
                return [a, b, c, d];
            }
            1 | 3 => {
                let a = self.position.left();
                let b = a.right();
                let c = b.right();
                let d = c.right();
                return [a, b, c, d];
            }
            _ => panic!("illegal rotation value: {}", self.rotation),
        }
    }

    fn get_j_points(&self) -> [Point; 4] {
        match self.rotation {
            0 => {
                let a = self.position.up();
                let b = a.down();
                let c = b.down();
                let d = c.left();
                return [a, b, c, d];
            }
            1 => {
                let a = self.position.right();
                let b = a.left();
                let c = b.left();
                let d = c.up();
                return [a, b, c, d];
            }
            2 => {
                let a = self.position.down();
                let b = a.up();
                let c = b.up();
                let d = c.right();
                return [a, b, c, d];
            }
            3 => {
                let a = self.position.left();
                let b = a.right();
                let c = b.right();
                let d = c.down();
                return [a, b, c, d];
            }
            _ => panic!("illegal rotation value: {}", self.rotation),
        }
    }

    fn get_l_points(&self) -> [Point; 4] {
        match self.rotation {
            0 => {
                let a = self.position.up();
                let b = a.down();
                let c = b.down();
                let d = c.right();
                return [a, b, c, d];
            }
            1 => {
                let a = self.position.right();
                let b = a.left();
                let c = b.left();
                let d = c.down();
                return [a, b, c, d];
            }
            2 => {
                let a = self.position.down();
                let b = a.up();
                let c = b.up();
                let d = c.left();
                return [a, b, c, d];
            }
            3 => {
                let a = self.position.left();
                let b = a.right();
                let c = b.right();
                let d = c.up();
                return [a, b, c, d];
            }
            _ => panic!("illegal rotation value: {}", self.rotation),
        }
    }

    fn get_o_points(&self) -> [Point; 4] {
        match self.rotation {
            0 | 1 | 2 | 3 => {
                let a = self.position.clone();
                let b = a.right();
                let c = b.down();
                let d = c.left();
                return [a, b, c, d];
            }
            _ => panic!("illegal rotation value: {}", self.rotation),
        }
    }

    fn get_s_points(&self) -> [Point; 4] {
        match self.rotation {
            0 | 2 => {
                let a = self.position.right();
                let b = a.left();
                let c = b.down();
                let d = c.left();
                return [a, b, c, d];
            }
            1 | 3 => {
                let a = self.position.up();
                let b = a.down();
                let c = b.right();
                let d = c.down();
                return [a, b, c, d];
            }
            _ => panic!("illegal rotation value: {}", self.rotation),
        }
    }

    fn get_t_points(&self) -> [Point; 4] {
        match self.rotation {
            0 => {
                let a = self.position.down();
                let b = a.up();
                let c = b.left();
                let d = b.right();
                return [a, b, c, d];
            }
            1 => {
                let a = self.position.left();
                let b = a.right();
                let c = b.up();
                let d = b.down();
                return [a, b, c, d];
            }
            2 => {
                let a = self.position.up();
                let b = a.down();
                let c = b.right();
                let d = b.left();
                return [a, b, c, d];
            }
            3 => {
                let a = self.position.right();
                let b = a.left();
                let c = b.down();
                let d = b.up();
                return [a, b, c, d];
            }
            _ => panic!("illegal rotation value: {}", self.rotation),
        }
    }

    fn get_z_points(&self) -> [Point; 4] {
        match self.rotation {
            0 | 2 => {
                let a = self.position.left();
                let b = a.right();
                let c = b.down();
                let d = c.right();
                return [a, b, c, d];
            }
            1 | 3 => {
                let a = self.position.up();
                let b = a.down();
                let c = b.left();
                let d = c.down();
                return [a, b, c, d];
            }
            _ => panic!("illegal rotation value: {}", self.rotation),
        }
    }

    pub fn is_collision(&self) -> bool {
        let ps = self.get_tetromino_points();

        for p in ps {
            if p.x < WELL_LEFT {
                return true;
            }

            if p.x >= WELL_LEFT + self.well.width {
                return true;
            }

            if p.y >= WELL_TOP + self.well.height {
                return true;
            }

            if p.y < WELL_TOP {
                continue;
            }

            let i = (self.well.width * (p.y - WELL_TOP) + (p.x - WELL_LEFT)) as usize;
            if self.well.colors[i].is_some() {
                return true;
            }
        }

        false
    }

    pub fn draw(&self, canvas: &mut HalfblockCanvas) {
        let ps = self.get_tetromino_points();
        canvas.draw_pixel(&ps[0], &self.colors[0]);
        canvas.draw_pixel(&ps[1], &self.colors[1]);
        canvas.draw_pixel(&ps[2], &self.colors[2]);
        canvas.draw_pixel(&ps[3], &self.colors[3]);
    }

    pub fn move_left(&mut self) {
        self.position.x -= 1;
        if self.is_collision() {
            self.position.x += 1;
        }
    }

    pub fn move_right(&mut self) {
        self.position.x += 1;
        if self.is_collision() {
            self.position.x -= 1;
        }
    }

    pub fn move_down(&mut self) -> bool {
        self.position.y += 1;
        if self.is_collision() {
            self.position.y -= 1;
            return true;
        }

        false
    }

    pub fn rotate(&mut self) {
        let old_rotation = self.rotation;

        self.rotation += 1;
        if self.rotation > 3 {
            self.rotation = 0;
        }

        if self.is_collision() {
            self.rotation = old_rotation;
        }
    }
}

use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::Canvas;
use term2d::view::canvas::halfblock::HalfblockCanvas;

use super::constants::{WELL_LEFT, WELL_TOP};
use super::well::Well;

pub enum TetrominoKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub struct Tetromino {
    color: Rgba,
    kind: TetrominoKind,
    position: Point,
    rotation: u8,
    well: Well,
}

impl Tetromino {
    pub fn new(kind: TetrominoKind, well: Well) -> Self {
        Self {
            color: Rgba::green(),
            kind,
            position: Point::new(4, 3),
            rotation: 0,
            well,
        }
    }

    fn get_tetromino_points(&self) -> [Point; 4] {
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
        [
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
        ]
    }

    fn get_s_points(&self) -> [Point; 4] {
        [
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
        ]
    }

    fn get_t_points(&self) -> [Point; 4] {
        [
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
        ]
    }

    fn get_z_points(&self) -> [Point; 4] {
        [
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(0, 0),
        ]
    }

    fn is_collision(&self) -> bool {
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

            let i = (self.well.width * (p.y - WELL_TOP) + (p.x - WELL_LEFT)) as usize;
            if self.well.colors[i].is_some() {
                return true;
            }
        }

        false
    }

    pub fn draw(&self, canvas: &mut HalfblockCanvas) {
        let ps = self.get_tetromino_points();
        canvas.draw_pixel(&ps[0], &self.color);
        canvas.draw_pixel(&ps[1], &self.color);
        canvas.draw_pixel(&ps[2], &self.color);
        canvas.draw_pixel(&ps[3], &self.color);
    }

    pub fn drop(&mut self) {
        todo!()
    }

    pub fn is_falling(&self) -> bool {
        true
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

    pub fn move_down(&mut self) {
        self.position.y += 1;
        if self.is_collision() {
            self.position.y -= 1;
        }
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

use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::Canvas;

use super::constants::WELL_LEFT;
use super::constants::WELL_TOP;
use super::tetromino::Tetromino;
use super::well::Well;

pub struct TetrominoI {
    color: Rgba,
    position: Point,
    rotation: u8,
    well: Well,
}

impl TetrominoI {
    pub fn new(well: Well) -> Self {
        Self {
            color: Rgba::green(),
            position: Point::new(4, 3),
            rotation: 0,
            well,
        }
    }

    fn get_tetromino_points(&self) -> [Point; 4] {
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

    fn is_collision(&self) -> bool {
        let ps = self.get_tetromino_points();

        for p in ps {
            let i = (self.well.width * (p.y - WELL_TOP) + (p.x - WELL_LEFT)) as usize;
            if self.well.colors[i].is_some() {
                return true;
            }

            if p.x < WELL_LEFT {
                return true;
            }

            if p.x >= WELL_LEFT + self.well.width {
                return true;
            }

            if p.y >= WELL_TOP + self.well.height {
                return true;
            }
        }

        false
    }
}

impl Tetromino for TetrominoI {
    fn draw(&self, canvas: &mut term2d::view::canvas::halfblock::HalfblockCanvas) {
        let ps = self.get_tetromino_points();
        canvas.draw_pixel(&ps[0], &self.color);
        canvas.draw_pixel(&ps[1], &self.color);
        canvas.draw_pixel(&ps[2], &self.color);
        canvas.draw_pixel(&ps[3], &self.color);
    }

    fn drop(&mut self) {
        todo!()
    }

    fn is_falling(&self) -> bool {
        true
    }

    fn move_left(&mut self) {
        self.position.x -= 1;
        if self.is_collision() {
            self.position.x += 1;
        }
    }

    fn move_right(&mut self) {
        self.position.x += 1;
        if self.is_collision() {
            self.position.x -= 1;
        }
    }

    fn move_down(&mut self) {
        self.position.y += 1;
        if self.is_collision() {
            self.position.y -= 1;
        }
    }

    fn rotate(&mut self) {
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

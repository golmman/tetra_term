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

    fn is_collision(&self) -> bool {
        if self.rotation == 0 || self.rotation == 2 {
            if self.position.x < WELL_LEFT {
                return true;
            }
            if self.position.x >= WELL_LEFT + self.well.width {
                return true;
            }
            if self.position.y >= WELL_TOP + self.well.height - 3 {
                return true;
            }

            let p0 = &Point::new(self.position.x - WELL_LEFT, self.position.y - WELL_TOP);
            let p1 = &p0.down();
            let p2 = &p1.down();
            let p3 = &p2.down();
            let ps = vec![p0, p1, p2, p3];

            for p in ps {
                let i = (self.well.width * p.y + p.x) as usize;
                if self.well.colors[i].is_some() {
                    return true;
                }
            }
        }

        if self.rotation == 1 || self.rotation == 3 {
            if self.position.x < WELL_LEFT {
                return true;
            }
            if self.position.x >= WELL_LEFT + self.well.width - 3 {
                return true;
            }
            if self.position.y >= WELL_TOP + self.well.height {
                return true;
            }

            let p0 = &Point::new(self.position.x - WELL_LEFT, self.position.y - WELL_TOP);
            let p1 = &p0.right();
            let p2 = &p1.right();
            let p3 = &p2.right();
            let ps = vec![p0, p1, p2, p3];

            for p in ps {
                let i = (self.well.width * p.y + p.x) as usize;
                if self.well.colors[i].is_some() {
                    return true;
                }
            }
        }

        false
    }
}

impl Tetromino for TetrominoI {
    fn draw(&self, canvas: &mut term2d::view::canvas::halfblock::HalfblockCanvas) {
        match self.rotation {
            0 => {
                let a = &self.position;
                let b = &a.down();
                let c = &b.down();
                let d = &c.down();
                canvas.draw_pixel(a, &self.color);
                canvas.draw_pixel(b, &self.color);
                canvas.draw_pixel(c, &self.color);
                canvas.draw_pixel(d, &self.color);
            }
            1 => {
                let a = &self.position;
                let b = &a.right();
                let c = &b.right();
                let d = &c.right();
                canvas.draw_pixel(a, &self.color);
                canvas.draw_pixel(b, &self.color);
                canvas.draw_pixel(c, &self.color);
                canvas.draw_pixel(d, &self.color);
            }
            2 => {
                let a = &self.position;
                let b = &a.down();
                let c = &b.down();
                let d = &c.down();
                canvas.draw_pixel(a, &self.color);
                canvas.draw_pixel(b, &self.color);
                canvas.draw_pixel(c, &self.color);
                canvas.draw_pixel(d, &self.color);
            }
            3 => {
                let a = &self.position;
                let b = &a.right();
                let c = &b.right();
                let d = &c.right();
                canvas.draw_pixel(a, &self.color);
                canvas.draw_pixel(b, &self.color);
                canvas.draw_pixel(c, &self.color);
                canvas.draw_pixel(d, &self.color);
            }
            _ => panic!("illegal rotation value"),
        }
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
        self.rotation += 1;
        if self.rotation > 3 {
            self.rotation = 0;
        }

        if self.rotation == 0 || self.rotation == 2 {
            self.position.x += 2;
            self.position.y -= 1;
        }

        if self.rotation == 1 || self.rotation == 3 {
            self.position.x -= 2;
            self.position.y += 1;
        }
    }
}

use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::Canvas;

use super::tetromino::Tetromino;
use super::well::Well;

pub struct TetrominoJ {
    color: Rgba,
    position: Point,
    rotation: u8,
    well: Well,
}

impl TetrominoJ {
    pub fn new(well: Well) -> Self {
        Self {
            color: Rgba::red(),
            position: Point::new(4, 3),
            rotation: 0,
            well,
        }
    }


    fn get_tetromino_points(&self) -> [Point; 4] {
        match self.rotation {
            0 => {
                let a = self.position.right();
                let b = a.down();
                let c = b.down();
                let d = c.left();
                return [a, b, c, d];
            }
            1 => {
                let a = self.position.clone();
                let b = a.down();
                let c = b.right();
                let d = c.right();
                return [a, b, c, d];
            }
            2 => {
                let a = self.position.clone();
                let b = a.right();
                let c = a.down();
                let d = c.down();
                return [a, b, c, d];
            }
            3 => {
                let a = self.position.clone();
                let b = a.right();
                let c = b.right();
                let d = c.down();
                return [a, b, c, d];
            }
            _ => panic!("illegal rotation value: {}", self.rotation),
        }
    }
}

impl Tetromino for TetrominoJ {
    fn draw(&self, canvas: &mut term2d::view::canvas::halfblock::HalfblockCanvas) {
        match self.rotation {
            0 => {
                let a = &self.position.right();
                let b = &a.down();
                let c = &b.down();
                let d = &c.left();
                canvas.draw_pixel(a, &self.color);
                canvas.draw_pixel(b, &self.color);
                canvas.draw_pixel(c, &self.color);
                canvas.draw_pixel(d, &self.color);
            }
            1 => {
                let a = &self.position;
                let b = &a.down();
                let c = &b.right();
                let d = &c.right();
                canvas.draw_pixel(a, &self.color);
                canvas.draw_pixel(b, &self.color);
                canvas.draw_pixel(c, &self.color);
                canvas.draw_pixel(d, &self.color);
            }
            2 => {
                let a = &self.position;
                let b = &a.right();
                let c = &a.down();
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
                let d = &c.down();
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
        todo!()
    }

    fn move_left(&mut self) {
        todo!()
    }

    fn move_right(&mut self) {
        todo!()
    }

    fn move_down(&mut self) {
        todo!()
    }

    fn rotate(&mut self) {
        self.rotation += 1;
        if self.rotation > 3 {
            self.rotation = 0;
        }
    }
}

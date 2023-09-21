use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::Canvas;

use super::tetromino::Tetromino;

pub struct TetrominoJ {
    color: Rgba,
    position: Point,
    rotation: u8,
}

impl TetrominoJ {
    pub fn new() -> Self {
        Self {
            color: Rgba::red(),
            position: Point::new(40, 5),
            rotation: 0,
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

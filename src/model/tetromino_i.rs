use term2d::model::point::Point;
use term2d::model::rgba::Rgba;
use term2d::view::canvas::Canvas;

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
            position: Point::new(50, 5),
            rotation: 0,
            well,
        }
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
    }

    fn move_right(&mut self) {
        self.position.x += 1;
    }

    fn move_down(&mut self) {
        self.position.y += 1;
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

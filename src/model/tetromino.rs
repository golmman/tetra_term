use term2d::view::canvas::halfblock::HalfblockCanvas;

pub trait Tetromino {
    fn draw(&self, canvas: &mut HalfblockCanvas);
    fn drop(&mut self);
    fn is_falling(&self) -> bool;
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_down(&mut self);
    fn rotate(&mut self);
}

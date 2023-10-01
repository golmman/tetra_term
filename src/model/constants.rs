use term2d::model::color::Color;
use term2d::model::rgba::Rgba;

pub const WELL_LEFT: i32 = 1;
pub const WELL_TOP: i32 = 2;
pub const INFO_WIDTH: i32 = 5;

const HEX_BACKGROUND: &'static str = "240d35";

pub const RGBA_BACKGROUND: Rgba = Rgba::from_hex(HEX_BACKGROUND);
pub const COLOR_TEXT: Color = Color {
    bg: Rgba::from_hex(HEX_BACKGROUND),
    fg: Rgba::new(200, 200, 180, 255),
};

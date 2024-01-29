use sdl2::pixels::Color;

pub const WIDTH: usize = 960;
pub const HEIGHT: usize = 540;

pub type Screen = [[Color; WIDTH]; HEIGHT];

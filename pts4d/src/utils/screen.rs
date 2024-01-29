use sdl2::pixels::Color;

pub(crate) const WIDTH: usize = 960;
pub(crate) const HEIGHT: usize = 540;

pub(crate) type Screen = [[Color; WIDTH]; HEIGHT];

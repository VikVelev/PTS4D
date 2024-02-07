use cgmath::Vector3;

pub const WIDTH: usize = 960;
pub const HEIGHT: usize = 540;

pub type Screen = [[Vector3<f32>; WIDTH]; HEIGHT];

use cgmath::Vector3;

pub const WIDTH: usize = 178*3;
pub const HEIGHT: usize = 100*3;

pub type Screen = [[Vector3<f32>; WIDTH]; HEIGHT];

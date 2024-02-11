use cgmath::Vector3;

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 360;

pub type Screen = [[Vector3<f32>; WIDTH]; HEIGHT];

use cgmath::Vector3;

const SCALE: usize = 3;
// 1.78 is the ratio 16 : 9
pub const WIDTH: usize = 178 * SCALE;
pub const HEIGHT: usize = 100 * SCALE;

pub type Screen = Vec<[Vector3<f32>; WIDTH]>;
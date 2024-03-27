use cgmath::Vector3;

const SCALE: usize = 4;
// 1.78 is the ratio 16 : 9
pub const RATIO: f32 = 1.00;
pub const WIDTH: usize = ((RATIO * 100.0) as usize) * SCALE;
pub const HEIGHT: usize = 100 * SCALE;

pub type Screen = Vec<Vec<Vector3<f32>>>;

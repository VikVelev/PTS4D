use cgmath::Vector3;

const SCALE: usize = 4;
// 1.78 is the ratio 16 : 9
const ASPECT: usize = 100;
pub const WIDTH: usize = ASPECT * SCALE;
pub const HEIGHT: usize = 100 * SCALE;

pub type Screen = Vec<Vec<Vector3<f32>>>;

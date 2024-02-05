use crate::scene::camera::Camera;
use crate::object::object::Object;

const NUM_OF_OBJECTS: usize = 1;

pub struct Scene {
    pub objects: [Object; NUM_OF_OBJECTS],
    pub camera: Camera,
}
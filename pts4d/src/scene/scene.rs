use crate::screen::Screen;
use crate::scene::camera::Camera;
use crate::object::object::Object;

const NUM_OF_OBJECTS: usize = 6;

pub struct Scene {
    screen: Screen,
    objects: [Object; NUM_OF_OBJECTS], // Objects
    camera: Camera,
}
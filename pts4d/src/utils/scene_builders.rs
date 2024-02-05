
use crate::scene::scene::Scene;
use crate::object::object::Object;
use crate::scene::camera::Camera;

pub fn generate_scene() -> Scene {
    return Scene {
        objects: [Object {}; 1],
        camera: Camera {},
    };
}
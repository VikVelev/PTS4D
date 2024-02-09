use cgmath::{Array, ElementWise, Vector3};

use crate::object::object::{Object, Sphere};
use crate::scene::camera::Camera;
use crate::utils::vector_utils::Ray;

const NUM_OF_OBJECTS: usize = 1;

pub struct Scene {
    pub objects: [Sphere; NUM_OF_OBJECTS],
    pub camera: Camera,
}

impl Scene {
    pub fn shoot_ray(&self, x: f32, y: f32) -> Ray {
        let cam = &self.camera;

        return Ray {
            origin: cam.origin,
            direction: cam.lower_left_corner + (x * cam.horizontal) + (y * cam.vertical)
                - cam.origin,
        };
    }
}

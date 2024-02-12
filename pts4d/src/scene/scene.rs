use crate::materials::material::Lambertian;
use crate::object::object::Mesh;
use crate::scene::camera::Camera;
use crate::utils::vector_utils::Ray;

const NUM_OF_OBJECTS: usize = 1;

pub struct Scene {
    // pub objects: [Sphere<Lambertian>; NUM_OF_OBJECTS],
    pub objects: [Mesh<Lambertian>; NUM_OF_OBJECTS],
    pub camera: Camera,
}

impl Scene {
    pub fn shoot_ray(&self, x: f32, y: f32) -> Ray {
        return self.camera.shoot_ray(x, y);
    }
}

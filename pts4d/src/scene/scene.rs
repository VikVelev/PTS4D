use crate::object::mesh::Mesh;
use crate::object::pixel_grid::PixelGrid;
use crate::object::sphere::Sphere;
use crate::scene::camera::Camera;
use crate::utils::vector_utils::Ray;

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub spheres: Vec<Sphere>,
    pub grids: Vec<PixelGrid>,
    pub camera: Camera,
}

impl Scene {
    pub fn build_complex_scene(meshes: Vec<Mesh>, spheres: Vec<Sphere>, camera: Camera) -> Scene {
        return Scene {
            meshes,
            spheres,
            grids: Vec::new(),
            camera,
        };
    }

    pub fn shoot_ray(&self, x: f32, y: f32) -> Ray {
        return self.camera.shoot_ray(x, y);
    }
}

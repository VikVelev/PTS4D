use crate::materials::material::Lambertian;
use crate::object::object::{Mesh, Sphere};
use crate::scene::camera::Camera;
use crate::utils::vector_utils::Ray;

const NUM_OF_OBJECTS: usize = 1;

pub struct Scene {
    pub meshes: Vec<Mesh<Lambertian>>,
    pub spheres: Vec<Sphere<Lambertian>>,
    pub camera: Camera,
}

impl Scene {
    pub fn build_complex_scene(
        meshes: Vec<Mesh<Lambertian>>,
        spheres: Vec<Sphere<Lambertian>>,
        camera: Camera,
    ) -> Scene {
        return Scene {
            meshes,
            spheres,
            camera,
        };
    }

    pub fn build_mesh_scene(meshes: Vec<Mesh<Lambertian>>, camera: Camera) -> Scene {
        return Scene {
            meshes,
            spheres: Vec::new(), // Empty
            camera,
        };
    }

    pub fn build_sphere_scene(spheres: Vec<Sphere<Lambertian>>, camera: Camera) -> Scene {
        return Scene {
            meshes: Vec::new(), // Empty
            spheres,
            camera,
        };
    }

    pub fn shoot_ray(&self, x: f32, y: f32) -> Ray {
        return self.camera.shoot_ray(x, y);
    }
}

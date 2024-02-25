use crate::materials::material::Metallic;
use crate::object::mesh::Mesh;
use crate::object::sphere::Sphere;
use crate::scene::camera::Camera;
use crate::utils::vector_utils::Ray;

pub struct Scene {
    pub meshes: Vec<Mesh<Metallic>>,
    pub spheres: Vec<Sphere<Metallic>>,
    pub camera: Camera,
}

impl Scene {
    pub fn build_complex_scene(
        meshes: Vec<Mesh<Metallic>>,
        spheres: Vec<Sphere<Metallic>>,
        camera: Camera,
    ) -> Scene {
        return Scene {
            meshes,
            spheres,
            camera,
        };
    }

    pub fn _build_mesh_scene(meshes: Vec<Mesh<Metallic>>, camera: Camera) -> Scene {
        return Scene {
            meshes,
            spheres: Vec::new(), // Empty
            camera,
        };
    }

    pub fn _build_sphere_scene(spheres: Vec<Sphere<Metallic>>, camera: Camera) -> Scene {
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

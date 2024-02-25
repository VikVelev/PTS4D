use std::fs;

use cgmath::{InnerSpace, Vector3, VectorSpace};
use wavefront_obj::obj::ObjSet;

use crate::materials::material::{Lambertian, Metallic};
use crate::object::mesh::Mesh;
use crate::object::sphere::Sphere;
use crate::scene::camera::Camera;
use crate::scene::scene::Scene;
use crate::scene::screen::{HEIGHT, WIDTH};

use super::vector_utils::Ray;

// Loads an obj file into memory and parses it into an ObjSet
pub fn load_and_parse_obj(path: &str) -> ObjSet {
    let obj_string = fs::read_to_string(path);
    if obj_string.is_err() {
        panic!("There was an error opening and reading '{}'", path);
    }

    let loaded_obj = wavefront_obj::obj::parse(obj_string.unwrap());
    if loaded_obj.is_err() {
        panic!("There was an error parsing '{}'", path);
    }

    return loaded_obj.unwrap();
}

// Creates a scene including complex polygon models.
pub fn generate_polygon_scene(path: &str) -> Scene {
    let mesh = load_and_parse_obj(path);
    let look_from = Vector3::new(5.0, 2.0, 5.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, -1.0, 0.0); // TODO: WTF?
    let camera: Camera = Camera::new(HEIGHT as f32, WIDTH as f32, 40.0, look_from, look_at, up);

    let loaded_mesh = Mesh::new(
        mesh,
        Metallic {
            albedo: Vector3::new(0.55, 0.55, 0.55),
            fuzz: 0.0,
        },
    );

    let ground_sphere = Sphere::new(
        Vector3::new(5.0, 0.0, 0.0),
        3.0,
        Metallic {
            albedo: Vector3::new(0.1, 0.7, 0.1),
            fuzz: 0.3,
        },
    );

    return Scene::build_complex_scene(vec![loaded_mesh], vec![ground_sphere], camera);
}

pub fn _generate_sphere_scene() -> Scene {
    let look_from = Vector3::new(0.0, 5.0, 30.0);
    let look_at = Vector3::new(0.0, 5.0, 0.0);
    let up = Vector3::new(0.0, -1.0, 0.0); // TODO: WTF?
    let _camera: Camera = Camera::new(HEIGHT as f32, WIDTH as f32, 40.0, look_from, look_at, up);

    let _ground_sphere = Sphere::new(
        Vector3::new(0.0, 5.0, 0.0),
        5.0,
        Lambertian {
            albedo: Vector3::new(1.0, 0.0, 0.0),
        },
    );

    let _main_sphere = Sphere::new(
        Vector3::new(0.0, -500.0, 0.0),
        500.0,
        Lambertian {
            albedo: Vector3::new(0.9, 0.9, 1.0),
        },
    );

    todo!();
    // return Scene::_build_sphere_scene(vec![main_sphere, ground_sphere], camera);
}

pub fn generate_sky(ray: &Ray) -> Vector3<f32> {
    let t = (0.5) * (ray.direction.normalize().y + 1.0);
    let white = Vector3::new(1.0, 1.0, 1.0);
    let blueish = Vector3::new(0.25, 0.75, 1.0);
    // Lerp gradient from white to blue-ish
    return white.lerp(blueish, t);
}

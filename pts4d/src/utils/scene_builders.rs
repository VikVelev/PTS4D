use std::fs;

use cgmath::{InnerSpace, Vector3, VectorSpace};
use wavefront_obj::obj::ObjSet;

use crate::materials::material::Lambertian;
use crate::object::object::{Mesh, Sphere};
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
    let look_from = Vector3 {
        x: 5.0,
        y: 2.0,
        z: 5.0,
    };
    let look_at = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let up = Vector3 {
        x: 0.0,
        y: -1.0, // TODO: wtf???
        z: 0.0,
    };
    let camera: Camera = Camera::new(HEIGHT as f32, WIDTH as f32, 40.0, look_from, look_at, up);

    return Scene {
        objects: [Mesh {
            geometry: mesh,
            material: Lambertian {
                albedo: Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        }],
        camera,
    };
}

pub fn _generate_scene() -> Scene {
    let look_from = Vector3 {
        x: 0.0,
        y: 5.0,
        z: 30.0,
    };
    let look_at = Vector3 {
        x: 0.0,
        y: 5.0,
        z: 0.0,
    };
    let up = Vector3 {
        x: 0.0,
        y: -1.0, // TODO: wtf???
        z: 0.0,
    };
    let _camera: Camera = Camera::new(HEIGHT as f32, WIDTH as f32, 40.0, look_from, look_at, up);

    let _ground_sphere = Sphere {
        center: Vector3 {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        },
        radius: 5.0,
        material: Lambertian {
            albedo: Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
    };

    let _main_sphere = Sphere {
        center: Vector3 {
            x: 0.0,
            y: -500.0,
            z: 0.0,
        },
        radius: 500.0,
        material: Lambertian {
            albedo: Vector3 {
                x: 0.9,
                y: 0.9,
                z: 0.1,
            },
        },
    };
    todo!();
    // return Scene {
    //     objects: [main_sphere, ground_sphere],
    //     camera,
    // };
}

pub fn generate_sky(ray: &Ray) -> Vector3<f32> {
    let t = (0.5) * (ray.direction.normalize().y + 1.0);
    let white = Vector3 {
        // white
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let blueish = Vector3 {
        // blue-ish
        x: 0.25,
        y: 0.75,
        z: 1.0,
    };
    // Lerp gradient from white to blue-ish
    return white.lerp(blueish, t);
}

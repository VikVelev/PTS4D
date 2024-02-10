use cgmath::{InnerSpace, Vector3};

use crate::materials::material::Metallic;
use crate::object::object::Sphere;
use crate::scene::camera::{construct_camera, Camera};
use crate::scene::scene::Scene;
use crate::scene::screen::{HEIGHT, WIDTH};

use super::vector_utils::Ray;

pub fn generate_scene() -> Scene {
    let look_from = Vector3 {
        x: 2.0,
        y: 5.0,
        z: -10.0,
    };
    let look_at = Vector3 {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    let up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let camera: Camera =
        construct_camera(look_from, look_at, up, 30.0, WIDTH as f32 / HEIGHT as f32);
    let sphere_mat = Metallic {
        color: Vector3 {
            x: 160.0,
            y: 60.0,
            z: 60.0,
        },
    };

    return Scene {
        objects: [Sphere {
            center: look_at,
            radius: 0.5,
            material: sphere_mat,
        }; 1],
        camera,
    };
}

pub fn generate_sky(ray: &Ray) -> Vector3<f32> {
    let t = (0.5) * (ray.direction.normalize().y + 1.0);
    // Lerp gradient from white to blue-ish
    return (1.0 - t)
        * Vector3 {
            // white
            x: 255.0,
            y: 255.0,
            z: 255.0,
        }
        + t * Vector3 {
            // blue-ish
            x: 128.0,
            y: 200.0,
            z: 255.0,
        };
}

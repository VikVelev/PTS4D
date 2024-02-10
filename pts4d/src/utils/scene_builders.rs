use cgmath::Vector3;

use crate::materials::material::Material;
use crate::object::object::{Object, Sphere};
use crate::scene::camera::{construct_camera, Camera};
use crate::scene::scene::Scene;
use crate::scene::screen::{HEIGHT, WIDTH};

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
    let camera: Camera = construct_camera(
        look_from,
        look_at,
        up,
        30.0,
        WIDTH as f32 / HEIGHT as f32,
    );
    return Scene {
        objects: [Sphere {
            center: look_at,
            radius: 0.5,
            material: Material { color: Vector3 { x: 160.0, y: 60.0, z: 60.0 }},
        }; 1],
        camera,
    };
}

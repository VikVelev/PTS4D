use cgmath::{InnerSpace, Vector3, VectorSpace};

use crate::materials::material::Lambertian;
use crate::object::object::Sphere;
use crate::scene::camera::Camera;
use crate::scene::scene::Scene;
use crate::scene::screen::{HEIGHT, WIDTH};

use super::vector_utils::Ray;

pub fn generate_scene() -> Scene {
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
    let camera: Camera = Camera::new(HEIGHT as f32, WIDTH as f32, 40.0, look_from, look_at, up);

    let ground_sphere = Sphere {
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

    let main_sphere = Sphere {
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

    return Scene {
        objects: [main_sphere, ground_sphere],
        camera,
    };
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

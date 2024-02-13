use cgmath::{InnerSpace, Vector3};
use rand::Rng;

use crate::materials::material::Reflective;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        return &self.origin + t * &self.direction;
    }
}

#[derive(Debug)]
pub struct Hit<'a, MaterialType: Reflective> {
    // Given a vector
    // a --- (p) ------> b
    // (p) denotes a constant where a ray is being intersected with something else.
    pub point_at_intersection: f32,

    // The hit point in 3d space.
    pub point: Vector3<f32>,

    // Normal vector denoting whether the hit came from the inside or outside
    // since by just a single point you have no idea.
    pub normal: Vector3<f32>,

    // Material, expressing what has been hit
    pub material: &'a MaterialType,
}

pub fn random_point_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    return loop {
        let temp =
            2.0 * Vector3 {
                x: rng.gen::<f32>(),
                y: rng.gen::<f32>(),
                z: rng.gen::<f32>(),
            } - Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };

        if temp.magnitude2() < 1.0 {
            break temp;
        };
    };
}

pub fn is_close_to_zero(vector: Vector3<f32>) -> bool {
    let limit = 0.000001;
    if vector.x.abs() < limit && vector.y.abs() < limit && vector.y.abs() < limit {
        return true;
    }
    return false;
}

pub fn correct_face_normal(ray: &Ray, normal: Vector3<f32>) -> Vector3<f32> {
    if ray.direction.dot(normal) < 0.0 {
        return normal;
    } else {
        return -normal;
    }
}

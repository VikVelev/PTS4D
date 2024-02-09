use cgmath::Vector3;

use crate::object::material::Material;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        return &self.origin + t * &self.direction;
    }
}

pub struct Intersect {
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
    pub material: Material
}
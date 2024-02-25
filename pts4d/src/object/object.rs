use cgmath::Vector3;

use crate::accel::aabb::AABB;
use crate::materials::material::Material;
use crate::utils::vector_utils::{Interval, Ray};

pub trait Hitable {
    // A function checking if the object implementing this trait
    // can be intersected by a specific ray, given some bounds
    //
    // Returns an Intersect object, which contains all necessary information to bounce / render.
    // Should return None if there is no intersection
    fn intersect(&self, ray: &Ray, bounds: Interval) -> Option<Hit>;
    fn bounding_box(&self) -> &AABB;
}

#[derive(Debug)]
pub struct Hit<'a> {
    // Given a vector
    // a --- (p) ------> b
    // (p) denotes a constant where a ray is being intersected with something else.
    pub point_at_intersection: f32,

    // The hit point in 3d space.
    pub point: Vector3<f32>,

    // Normal vector denoting whether the hit came from the inside or outside
    // since by just a single point you have no idea.
    pub normal: Vector3<f32>,

    pub is_facing_you: bool,

    // Material, expressing what has been hit
    pub material: &'a Material,
}

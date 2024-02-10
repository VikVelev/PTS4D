use crate::materials::material::Reflective;
use crate::utils::vector_utils::{Hit, Ray};

use cgmath::{dot, InnerSpace, Vector3};

pub trait Hitable: Sized {
    type Material: Reflective;
    // A function checking if the object implementing this trait
    // can be intersected by a specific ray, given some bounds
    //
    // Returns an Intersect object, which contains all necessary information to bounce / render.
    // Should return None if there is no intersection
    fn intersect(&self, ray: &Ray, bounds: (f32, f32)) -> Option<Hit<Self::Material>>
    where
        Self: Sized;
}

pub struct Sphere<T: Reflective> {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: T,
}

impl<Mat: Reflective> Hitable for Sphere<Mat> {
    fn intersect(&self, ray: &Ray, bounds: (f32, f32)) -> Option<Hit<Mat>> {
        // TODO: Check and rewrite math
        // Dirty garbage to get a circle rendering.

        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction); // || ray.direction ||^2
        let b = 2.0 * dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let x1 = (-b - discriminant.sqrt()) / (2.0 * a);
            if x1 < bounds.1 && x1 > bounds.0 {
                return Some(Hit {
                    point_at_intersection: x1,
                    point: ray.point_at(x1),
                    normal: correct_face_normal(ray, (ray.point_at(x1) - self.center) / self.radius),
                    material: &self.material,
                });
            }

            let x2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if x2 < bounds.1 && x2 > bounds.0 {
                return Some(Hit {
                    point_at_intersection: x2,
                    point: ray.point_at(x2),
                    normal: correct_face_normal(ray, (ray.point_at(x2) - self.center) / self.radius),
                    material: &self.material,
                });
            }
        }

        return None;
    }

    type Material = Mat;
}

fn correct_face_normal(ray: &Ray, normal: Vector3<f32>) -> Vector3<f32> {
    if ray.direction.dot(normal) < 0.0 {
        return normal;
    } else {
        return -normal;
    }
}

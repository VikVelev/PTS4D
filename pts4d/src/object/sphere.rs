use cgmath::{dot, InnerSpace, Vector3};

use crate::{accel::aabb::AABB, materials::material::Material, utils::vector_utils::{correct_face_normal, Interval, Ray}};

use super::object::{Hit, Hitable};

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
    bbox: AABB,
}

impl Sphere {
    fn compute_bounding_box(center: Vector3<f32>, radius: f32) -> AABB {
        let radius_vec = Vector3::new(radius, radius, radius);
        return AABB::new_from_diagonals(center - radius_vec, center + radius_vec);
    }

    pub fn new(center: Vector3<f32>, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            bbox: Sphere::compute_bounding_box(center, radius),
        }
    }
}

impl Hitable for Sphere {
    fn intersect(&self, ray: &Ray, bounds: Interval) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction); // || ray.direction ||^2
        let b = 2.0 * dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let x1 = (-b - discriminant.sqrt()) / (2.0 * a);
            if x1 < bounds.max && x1 > bounds.min {
                let ray_t = ray.point_at(x1);
                let normal = ((ray_t - self.center) / self.radius).normalize();
                return Some(Hit {
                    point_at_intersection: x1,
                    point: ray_t,
                    normal: correct_face_normal(ray, normal),
                    is_facing_you: ray.direction.dot(normal) < 0.0,
                    material: &self.material,
                });
            }

            let x2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if x2 < bounds.max && x2 > bounds.min {
                let ray_t = ray.point_at(x2);
                let normal = ((ray_t - self.center) / self.radius).normalize();
                return Some(Hit {
                    point_at_intersection: x2,
                    point: ray_t,
                    normal: correct_face_normal(ray, normal),
                    is_facing_you: ray.direction.dot(normal) < 0.0,
                    material: &self.material,
                });
            }
        }

        return None;
    }

    fn bounding_box(&self) -> &AABB {
        return &self.bbox;
    }
}
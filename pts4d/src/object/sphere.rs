use cgmath::{dot, Vector3};

use crate::{accel::aabb::AABB, materials::material::Reflective, utils::vector_utils::{correct_face_normal, Interval, Ray}};

use super::object::{Hit, Hitable};

pub struct Sphere<T: Reflective> {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: T,
    bbox: AABB,
}

impl<T: Reflective> Sphere<T> {
    fn compute_bounding_box(center: Vector3<f32>, radius: f32) -> AABB {
        let radius_vec = Vector3::new(radius, radius, radius);
        return AABB::new_from_diagonals(center - radius_vec, center + radius_vec);
    }

    pub fn new(center: Vector3<f32>, radius: f32, material: T) -> Sphere<T> {
        Sphere {
            center,
            radius,
            material,
            bbox: Sphere::<T>::compute_bounding_box(center, radius),
        }
    }
}

impl<Mat: Reflective> Hitable for Sphere<Mat> {
    fn intersect(&self, ray: &Ray, bounds: Interval) -> Option<Hit<Mat>> {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction); // || ray.direction ||^2
        let b = 2.0 * dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let x1 = (-b - discriminant.sqrt()) / (2.0 * a);
            if x1 < bounds.max && x1 > bounds.min {
                return Some(Hit {
                    point_at_intersection: x1,
                    point: ray.point_at(x1),
                    normal: correct_face_normal(
                        ray,
                        (ray.point_at(x1) - self.center) / self.radius,
                    ),
                    material: &self.material,
                });
            }

            let x2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if x2 < bounds.max && x2 > bounds.min {
                return Some(Hit {
                    point_at_intersection: x2,
                    point: ray.point_at(x2),
                    normal: correct_face_normal(
                        ray,
                        (ray.point_at(x2) - self.center) / self.radius,
                    ),
                    material: &self.material,
                });
            }
        }

        return None;
    }

    type Material = Mat;

    fn bounding_box(&self) -> &AABB {
        return &self.bbox;
    }
}
use cgmath::{InnerSpace, Vector3};

use crate::utils::vector_utils::{is_close_to_zero, random_point_in_unit_sphere, Hit, Ray};

pub trait Reflective {
    fn scatter(&self, ray_in: &Ray, hit: &Hit<impl Reflective>) -> Option<(Ray, Vector3<f32>)>;
}

pub struct Metallic {
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}

impl Reflective for Metallic {
    fn scatter(&self, ray_in: &Ray, hit: &Hit<impl Reflective>) -> Option<(Ray, Vector3<f32>)> {
        let reflected = reflect_vector(ray_in.direction.normalize(), hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: (reflected + self.fuzz * random_point_in_unit_sphere()).normalize()
        };

        if scattered.direction.dot(hit.normal) > 0.0 {
            return Some((scattered, self.albedo));
        }

        return None;
    }
}

pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Reflective for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &Hit<impl Reflective>) -> Option<(Ray, Vector3<f32>)> {
        let mut scatter_direction = hit.normal + random_point_in_unit_sphere().normalize();

        if is_close_to_zero(scatter_direction) {
            scatter_direction = hit.normal;
        }

        return Some((
            Ray {
                origin: hit.point,
                direction: scatter_direction,
            },
            self.albedo,
        ));
    }
}

#[inline]
fn reflect_vector(vec: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    return vec - 2.0 * vec.dot(normal) * normal;
}
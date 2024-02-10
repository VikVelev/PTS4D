use cgmath::Vector3;

use crate::utils::vector_utils::{random_point_in_unit_sphere, Hit, Ray};

pub trait Reflective {
    fn set_color(&mut self, color: Vector3<f32>) -> ();
    fn get_color(&self) -> Vector3<f32>;
    fn scatter(&self, ray_in: &Ray, hit: &Hit<impl Reflective>) -> Option<(Ray, Vector3<f32>)>;
}

#[derive(Copy, Clone)]
pub struct Metallic {
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}

impl Reflective for Metallic {
    fn set_color(&mut self, color: Vector3<f32>) -> () {
        self.albedo = color;
    }

    fn get_color(&self) -> Vector3<f32> {
        return self.albedo;
    }

    fn scatter(&self, _ray_in: &Ray, _hit: &Hit<impl Reflective>) -> Option<(Ray, Vector3<f32>)> {
        todo!()
    }
}

pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Reflective for Lambertian {
    fn set_color(&mut self, color: Vector3<f32>) -> () {
        self.albedo = color;
    }

    fn get_color(&self) -> Vector3<f32> {
        return self.albedo;
    }

    fn scatter(&self, _ray_in: &Ray, hit: &Hit<impl Reflective>) -> Option<(Ray, Vector3<f32>)> {
        let target = hit.point + hit.normal + random_point_in_unit_sphere();

        return Some((
            Ray {
                origin: hit.point,
                direction: target - hit.point,
            },
            self.albedo,
        ));
    }
}

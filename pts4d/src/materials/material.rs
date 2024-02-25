use cgmath::{InnerSpace, Vector3};

use crate::{
    object::object::Hit,
    utils::vector_utils::{is_close_to_zero, random_point_in_unit_sphere, Ray},
};

#[derive(Debug)]
pub enum Material {
    Lambertian(Vector3<f32>),    // albedo
    Metallic(Vector3<f32>, f32), // albedo, fuzz
    Dielectric(f32),             // refraction index
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Vector3<f32>)> {
        match self {
            Material::Lambertian(albedo) => lambertian_shading(ray_in, hit, *albedo),
            Material::Metallic(albedo, fuzz) => metallic_shading(ray_in, hit, *albedo, *fuzz),
            Material::Dielectric(refraction_index) => todo!(),
        }
    }
}

fn lambertian_shading(ray: &Ray, hit: &Hit, albedo: Vector3<f32>) -> Option<(Ray, Vector3<f32>)> {
    let mut scatter_direction = hit.normal + random_point_in_unit_sphere().normalize();

    if is_close_to_zero(scatter_direction) {
        scatter_direction = hit.normal;
    }

    return Some((
        Ray {
            origin: hit.point,
            direction: scatter_direction,
        },
        albedo,
    ));
}

fn metallic_shading(ray: &Ray, hit: &Hit, albedo: Vector3<f32>, fuzz: f32) -> Option<(Ray, Vector3<f32>)> {
    let reflected = reflect_vector(ray.direction.normalize(), hit.normal);
    let scattered = Ray {
        origin: hit.point,
        direction: (reflected + fuzz * random_point_in_unit_sphere()).normalize(),
    };

    if scattered.direction.dot(hit.normal) > 0.0 {
        return Some((scattered, albedo));
    }

    return None;
}

#[inline]
fn reflect_vector(vec: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    return vec - 2.0 * vec.dot(normal) * normal;
}

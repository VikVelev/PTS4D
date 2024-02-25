use cgmath::{InnerSpace, Vector3};
use rand::Rng;

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
            Material::Dielectric(refraction_index) => {
                dielectric_shading(ray_in, hit, *refraction_index)
            }
        }
    }
}

fn lambertian_shading(_ray: &Ray, hit: &Hit, albedo: Vector3<f32>) -> Option<(Ray, Vector3<f32>)> {
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

fn metallic_shading(
    ray: &Ray,
    hit: &Hit,
    albedo: Vector3<f32>,
    fuzz: f32,
) -> Option<(Ray, Vector3<f32>)> {
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

fn dielectric_shading(ray: &Ray, hit: &Hit, refraction_index: f32) -> Option<(Ray, Vector3<f32>)> {
    let mut rng = rand::thread_rng();

    let attenuation = Vector3::new(1.0, 1.0, 1.0);
    let refraction_ratio = if hit.is_facing_you {
        1.0 / refraction_index
    } else {
        refraction_index
    };
    
    let ray_direction_unit = ray.direction.normalize();
    let cos_theta = -ray_direction_unit.dot(hit.normal).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
 
    let scattered: Vector3<f32>;

    let cant_refract = refraction_ratio * sin_theta > 1.0;
    if cant_refract || (reflectance_schlick_approx(cos_theta, refraction_index) > rng.gen::<f32>()) {
        scattered = reflect_vector(ray_direction_unit, hit.normal);
    } else {
        scattered = refract_vector(ray_direction_unit, hit.normal, refraction_ratio)
    }

    return Some((
        Ray {
            origin: hit.point,
            direction: scattered,
        },
        attenuation,
    ));
}

#[inline]
fn reflectance_schlick_approx(cos: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0_2 = r0.powi(2);
    return r0_2 + (1.0 - r0_2) * f32::powf(1.0 - cos, 5.0);
}

#[inline]
fn reflect_vector(vec: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    return vec - 2.0 * vec.dot(normal) * normal;
}

#[inline]
fn refract_vector(vec: Vector3<f32>, normal: Vector3<f32>, refraction_ratio: f32) -> Vector3<f32> {
    // All vectors must be unit vectors
    let cos_theta = f32::min(-vec.dot(normal), 1.0);
    let r_out_perp = refraction_ratio * (vec + cos_theta * normal);
    let r_out_parallel = -f32::abs(1.0 - r_out_perp.magnitude2()).sqrt() * normal;

    return r_out_perp + r_out_parallel;
}

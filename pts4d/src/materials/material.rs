use std::collections::HashMap;

use cgmath::{InnerSpace, Vector3};
use rand::Rng;
use wavefront_obj::mtl::{Color, Material as WavefrontObjMaterial};
use crate::{
    object::object::Hit,
    utils::vector_utils::{is_close_to_zero, random_point_in_unit_sphere, Ray},
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Material {
    Diffuse(Vector3<f32>),       // albedo
    Metallic(Vector3<f32>, f32), // albedo, fuzz
    Dielectric(f32),             // refraction index
    Emissive(Vector3<f32>, f32), // albedo, intensity
    Texture(),                   // TODO Implement
    WavefrontObjMaterial(WavefrontObjMaterial), // everything
}

/**
 * MaterialSet is a set of materials shared within the whole geometry.
 * Allows for a single object to have multiple materials.
 */
pub struct MaterialSet {
    // A Map Material Name -> Material
    pub materials: HashMap<String, Material>,
}

impl MaterialSet {
    pub fn new_with_default_material(mat: Material) -> MaterialSet {
        let mut material_set = MaterialSet::new();
        material_set.add("__default__".to_string(), mat);
        return material_set;
    }

    pub fn new() -> MaterialSet {
        let new_hash_map: HashMap<String, Material> = HashMap::new();
        return MaterialSet {
            materials: new_hash_map,
        };
    }

    pub fn add(&mut self, key: String, value: Material) {
        self.materials.insert(key, value);
    }

    pub fn get(&self, key: &String) -> &Material {
        return self.materials.get(key).unwrap();
    }
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Vector3<f32>)> {
        match self {
            Material::Emissive(_, _) => None,
            Material::Diffuse(albedo) => lambertian_shading(ray_in, hit, *albedo),
            Material::Metallic(albedo, fuzz) => metallic_shading(ray_in, hit, *albedo, *fuzz),
            Material::Dielectric(refraction_index) => {
                dielectric_shading(ray_in, hit, *refraction_index)
            }
            Material::WavefrontObjMaterial(wavefront_mat) => {
                // TODO: Implement complex wavefront materials / phong / specular / emissive properties.
                return lambertian_shading(
                    ray_in,
                    hit,
                    wavefront_color_to_vector(wavefront_mat.color_diffuse),
                );
            }
            Material::Texture() => todo!(),
        }
    }

    pub fn emit(&self, ray_in: &Ray) -> Vector3<f32> {
        match self {
            Material::Emissive(color, intensity) => *intensity * *color,
            Material::WavefrontObjMaterial(wavefront_mat) => {
                if wavefront_mat.name != "Light" {
                    return Vector3::new(0.0, 0.0, 0.0)
                }

                return wavefront_color_to_vector(wavefront_mat.color_ambient) * 20.0;
            }
            _ => Vector3::new(0.0, 0.0, 0.0),
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

    let attenuation = Vector3::new(0.99, 0.99, 0.99);
    let refraction_ratio = if hit.is_facing_you {
        1.0 / refraction_index
    } else {
        refraction_index
    };

    let ray_direction_unit = ray.direction.normalize();
    let cos_theta = -(ray_direction_unit.dot(hit.normal).min(1.0));
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let scattered: Vector3<f32>;

    let cant_refract = refraction_ratio * sin_theta > 1.0;
    let fresnel_reflection =
        reflectance_schlick_approx(cos_theta, refraction_index) > rng.gen::<f32>();

    if cant_refract || fresnel_reflection {
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

#[inline]
fn wavefront_color_to_vector(color: Color) -> Vector3<f32> {
    return Vector3::new(color.r as f32, color.g as f32, color.b as f32);
}

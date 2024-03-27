use crate::accel::aabb::HitableAccelStructure;
use crate::object::object::{Hit, Hitable};
use crate::scene::scene::Scene;
use crate::scene::screen::Screen;
use crate::scene::screen::{HEIGHT, WIDTH};
use crate::utils::vector_utils::{Interval, Ray};
use cgmath::{ElementWise, Vector3};
use rayon::prelude::*;
use std::f32::MAX;

const MAX_DEPTH: i32 = 8;
pub const SAMPLES_PER_PIXEL: i32 = 35;
const MIN_T: f32 = 0.001;
const DEBUG_AABB: bool = false;

pub fn ray_trace(scene: &Scene, ray: &Ray) -> Vector3<f32> {
    return ray_trace_rec(scene, ray, 0);
}

// Recursively ray-trace until the number of bounces has reached MAX_DEPTH
pub fn ray_trace_rec(scene: &Scene, ray: &Ray, bounces: i32) -> Vector3<f32> {
    if bounces >= MAX_DEPTH {
        // Stare into the void!
        return Vector3::new(0.0, 0.0, 0.0);
    }

    let mut closest_hit: Option<Hit> = None;
    let mut closest_t = MAX;

    for obj in &scene.spheres {
        if obj
            .bounding_box()
            .intersect(ray, Interval::new(MIN_T, closest_t))
        {
            if DEBUG_AABB {
                return Vector3::new(1.0, 0.0, 0.0);
            }
            if let Some(hit) = cast_ray(obj, ray, closest_t) {
                closest_t = hit.point_at_intersection;
                closest_hit = Some(hit);
            }
        }
    }

    for obj in &scene.meshes {
        if obj
            .bounding_box()
            .intersect(ray, Interval::new(MIN_T, closest_t))
        {
            if DEBUG_AABB {
                return Vector3::new(0.0, 1.0, 0.0);
            }
            if let Some(hit) = cast_ray(obj, ray, closest_t) {
                closest_t = hit.point_at_intersection;
                closest_hit = Some(hit);
            }
        }
    }

    if let Some(hit) = closest_hit {
        let emitted_color = hit.material.emit(ray);

        if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
            let scattered_color =
                attenuation.mul_element_wise(ray_trace_rec(scene, &scattered, bounces + 1));
            return emitted_color + scattered_color;
        }

        return emitted_color;
    }

    // Stare into the void once again!
    return Vector3::new(0.0, 0.0, 0.0);
}

fn cast_ray<'a>(obj: &'a impl Hitable, ray: &'a Ray, closest_t: f32) -> Option<Hit<'a>> {
    // Cast a single ray and get the closest hit.
    let curr_obj = obj;
    let mut hit: Option<Hit> = None;

    let temp_closest_hit = curr_obj.intersect(ray, Interval::new(MIN_T, closest_t));
    if let Some(closest_hit) = temp_closest_hit {
        hit = Some(closest_hit);
    }

    return hit;
}

pub fn render_pass(scene: &Scene) -> Screen {
    let mut new_screen = vec![vec![Vector3::new(0.0, 0.0, 0.0); WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        // Create an array of all x coordinates for a specific row
        // Execute a single_pixel_pass on all of them and collect()
        // collect() preserves order so we can write it directly onto the screen
        new_screen[y] = (0..WIDTH)
            .collect::<Vec<_>>()
            .par_iter()
            .map(|x| single_pixel_pass(*x, y, scene))
            .collect();
    }
    return new_screen;
}

fn single_pixel_pass(x: usize, y: usize, scene: &Scene) -> Vector3<f32> {
    let mut color: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
    for _ in 0..SAMPLES_PER_PIXEL {
        let ray = scene.shoot_ray(x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32);
        color += ray_trace(scene, &ray);
    }

    return color / SAMPLES_PER_PIXEL as f32;
}

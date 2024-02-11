use crate::materials::material::{Lambertian, Reflective};
use crate::object::object::Hitable;
use crate::scene::scene::Scene;
use crate::scene::screen::Screen;
use crate::scene::screen::{HEIGHT, WIDTH};
use crate::utils::rendering_utils::preprocess_color;
use crate::utils::scene_builders::generate_sky;
use crate::utils::vector_utils::{Hit, Ray};
use cgmath::{ElementWise, Vector3};
use std::f32::MAX;

const MAX_DEPTH: i32 = 5;
const SAMPLES_PER_PIXEL: i32 = 1;

pub fn ray_trace(scene: &Scene, ray: &Ray) -> Vector3<f32> {
    return ray_trace_rec(scene, ray, 0);
}

// Recursively ray-trace until the number of bounces has reached MAX_DEPTH
pub fn ray_trace_rec(scene: &Scene, ray: &Ray, bounces: i32) -> Vector3<f32> {
    if bounces >= MAX_DEPTH {
        return Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    let mut hit: Option<Hit<Lambertian>> = None;
    let mut closest_intersection_point: f32 = MAX;

    for obj in &scene.objects {
        let main_sphere = obj;
        let temp_closest_hit = main_sphere.intersect(ray, (0.001, closest_intersection_point));
        if temp_closest_hit.is_some() {
            let closest_hit = temp_closest_hit.unwrap();
            closest_intersection_point = closest_hit.point_at_intersection;
            hit = Some(closest_hit);
        }
    }

    if hit.is_some() {
        let some_hit = hit.unwrap();
        let maybe_bounced_ray = some_hit.material.scatter(ray, &some_hit);

        if maybe_bounced_ray.is_some() {
            let (bounced_ray, attenuation) = maybe_bounced_ray.unwrap();
            return attenuation.mul_element_wise(ray_trace_rec(scene, &bounced_ray, bounces + 1));
        }

        return Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    
    return generate_sky(ray);
}

pub fn render_pass(scene: &Scene) -> Box<Screen> {
    let mut new_screen = Box::new(
        [[Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; WIDTH]; HEIGHT],
    );

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut color: Vector3<f32> = Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for _ in 0..SAMPLES_PER_PIXEL {
                let ray = scene.shoot_ray(x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32);
                color += ray_trace(scene, &ray);
            }
            new_screen[y][x] = preprocess_color(color, SAMPLES_PER_PIXEL);
        }
    }

    return new_screen;
}

use crate::materials::material::{Lambertian, Metallic, Reflective};
use crate::object::object::Hitable;
use crate::scene::scene::Scene;
use crate::scene::screen::Screen;
use crate::scene::screen::{HEIGHT, WIDTH};
use crate::utils::scene_builders::generate_sky;
use crate::utils::vector_utils::{Hit, Ray};
use cgmath::{ElementWise, Vector3};
use sdl2::libc::close;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f32::MAX;

const MAX_DEPTH: i32 = 10;
const SAMPLES_PER_PIXEL: i32 = 5;

pub fn ray_trace(scene: &Scene, ray: &Ray) -> Vector3<f32> {
    return ray_trace_rec(scene, ray, 0);
}

// Recursively ray-trace until the number of bounces has reached MAX_DEPTH
pub fn ray_trace_rec(scene: &Scene, ray: &Ray, bounces: i32) -> Vector3<f32> {
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


    if hit.is_some() && bounces < MAX_DEPTH {
        let some_hit = hit.unwrap();
        let maybe_bounced_ray = some_hit.material.scatter(ray, &some_hit);

        if maybe_bounced_ray.is_some() {
            let (bounced_ray, attenuation) = maybe_bounced_ray.unwrap();
            // return attenuation;
            return ray_trace_rec(scene, &bounced_ray, bounces + 1).mul_element_wise(attenuation);
        } else {
            return Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
    } else {
        return generate_sky(ray);
    }
}

pub fn render_pass(scene: &Scene, screen: Option<Box<Screen>>) -> Box<Screen> {
    let mut new_screen: Box<Screen>;

    if screen.is_some() {
        new_screen = screen.unwrap();
    } else {
        new_screen = Box::new(
            [[Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }; WIDTH]; HEIGHT],
        );
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut color: Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
            for _ in 0..SAMPLES_PER_PIXEL {
                let ray = scene.shoot_ray(x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32);
                color += ray_trace(scene, &ray) / SAMPLES_PER_PIXEL as f32;
            }
            new_screen[y][x] = color;
        }
    }

    return new_screen;
}

pub fn present_screen(screen: Box<Screen>, sdl_canvas: &mut Canvas<Window>) {
    for (y, row) in screen.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            sdl_canvas.set_draw_color(Color {
                r: pixel.x as u8,
                g: pixel.y as u8,
                b: pixel.z as u8,
                a: 0,
            });
            sdl_canvas
                .draw_point(Point::new(x as i32, y as i32))
                .unwrap();
        }
    }
    sdl_canvas.present();
}

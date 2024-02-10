use crate::materials::material::{Metallic, Reflective};
use crate::object::object::Hitable;
use crate::scene::scene::Scene;
use crate::scene::screen::Screen;
use crate::scene::screen::{HEIGHT, WIDTH};
use crate::utils::scene_builders::generate_sky;
use crate::utils::vector_utils::{Hit, Ray};
use cgmath::Vector3;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f32::MAX;

const MAX_DEPTH: i32 = 50;

pub fn ray_trace(scene: &Scene, ray: &Ray) -> Vector3<f32> {
    return ray_trace_rec(scene, ray, 0);
}

// Recursively ray-trace until the number of bounces has reached MAX_DEPTH
pub fn ray_trace_rec(scene: &Scene, ray: &Ray, bounces: i32) -> Vector3<f32> {
    let main_sphere = scene.objects.first();
    let mut hit: Option<Hit<Metallic>> = None;

    // TODO: Generalize this for all kinds of objects
    if main_sphere.is_some() {
        let sphere = main_sphere.unwrap();
        hit = sphere.intersect(ray, (0.001, MAX));
    }

    if hit.is_some() && bounces < MAX_DEPTH {
        let some_hit = hit.unwrap();
        // let bounced_ray = some_hit.material.scatter(ray);

        return some_hit.material.color;
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
            let ray = scene.shoot_ray(x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32);
            let color: Vector3<f32> = ray_trace(scene, &ray);
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

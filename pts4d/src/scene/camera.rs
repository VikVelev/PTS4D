use std::f32::consts::PI;

use cgmath::{InnerSpace, Vector3};

pub struct Camera {
    // TODO: Check and rewrite
    pub origin: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub w: Vector3<f32>,
}

pub fn construct_camera(
    look_from: Vector3<f32>,
    look_at: Vector3<f32>,
    up: Vector3<f32>,
    fov: f32,
    aspect_ratio: f32,
    aperature: f32,
) -> Camera {
    let lens_radius = aperature / 2.0;
    let theta = fov * PI / 180.0;

    let half_height = (theta / 2.0).tan();
    let half_width = aspect_ratio * half_height;

    let w = (look_from - look_at).normalize();
    let u = up.cross(w).normalize();
    let v = w.cross(u);
    let origin: Vector3<f32> = look_from;

    return Camera {
        origin,
        lower_left_corner: origin - half_width * u - half_height * v - w,
        horizontal: 2.0 * half_width * u,
        vertical: 2.0 * half_height * v,
        w,
        v,
        u,
    }
}

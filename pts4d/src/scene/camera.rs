use std::f32::consts::PI;

use cgmath::{InnerSpace, Vector3};
use rand::Rng;

use crate::utils::vector_utils::Ray;

pub struct CameraConfig {
    pub image_width: f32,
    pub image_height: f32,
    pub look_from: Vector3<f32>, // Where is the camera positioned
    pub look_at: Vector3<f32>,   // Where is the camera looking at?
    pub up: Vector3<f32>,        // which way is up
    pub fov: f32,                // Vertical FoV
}

pub struct Camera {
    // Public
    pub camera_config: CameraConfig,

    // Private
    origin: Vector3<f32>,
    first_pixel_location: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,
}

impl Camera {
    pub fn new(
        image_height: f32,
        image_width: f32,
        fov: f32,
        look_from: Vector3<f32>,
        look_at: Vector3<f32>,
        up: Vector3<f32>,
    ) -> Camera {
        let theta = fov * PI / 180.0;
        let aspect_ratio = image_width / image_height;

        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);
        let origin: Vector3<f32> = look_from;
  
        let pixel_delta_u = (viewport_width * u) / image_width;
        let pixel_delta_v = (viewport_height * -v) / image_height;

        let first_pixel_location =
            origin - (viewport_width / 2.0) * u - (viewport_height / 2.0) * v - w;
        
        return Camera {
            camera_config: CameraConfig {
                image_height,
                image_width,
                fov,
                look_from,
                look_at,
                up,
            },
            origin,
            first_pixel_location,
            horizontal: viewport_width * u,
            vertical: viewport_height * v,
            pixel_delta_u,
            pixel_delta_v,
        };
    }

    pub fn shoot_ray(&self, x: f32, y: f32) -> Ray {
        let pixel_center = self.first_pixel_location + (x * self.horizontal) + (y * self.vertical);
        let sample_pixel = pixel_center + self.sample_from_pixel_square();

        return Ray {
            origin: self.origin,
            direction: sample_pixel - self.origin,
        };
    }

    pub fn sample_from_pixel_square(&self) -> Vector3<f32> {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen::<f32>();
        let py: f32 = -0.5 + rng.gen::<f32>();

        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
}

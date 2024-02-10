use cgmath::Vector3;
use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::scene::screen::Screen;


pub fn preprocess_color(color: Vector3<f32>, samples_per_pixel: i32) -> Vector3<f32> {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scalar = 1.0 / (samples_per_pixel as f32);
    r *= scalar;
    g *= scalar;
    b *= scalar;

    // Gamma transform;
    r = r.sqrt();
    b = b.sqrt();
    g = g.sqrt();

    r = r.clamp(0.0, 0.999);
    g = g.clamp(0.0, 0.999);
    b = b.clamp(0.0, 0.999);

    return Vector3 { x: r * 255.0, y: g * 255.0, z: b * 255.0 }
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
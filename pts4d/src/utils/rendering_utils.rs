use std::ops::{Add, AddAssign};

use cgmath::{ElementWise, Vector3};
use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::scene::screen::{Screen, HEIGHT, WIDTH};

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

    return Vector3 {
        x: r * 255.0,
        y: g * 255.0,
        z: b * 255.0,
    };
}

pub fn present_screen(screen: Box<Screen>, sdl_canvas: &mut Canvas<Window>, iteration: i32) {
    for (y, row) in screen.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            sdl_canvas.set_draw_color(Color {
                r: (pixel.x / iteration as f32) as u8,
                g: (pixel.y / iteration as f32) as u8,
                b: (pixel.z / iteration as f32) as u8,
                a: 0,
            });
            sdl_canvas
                .draw_point(Point::new(x as i32, y as i32))
                .unwrap();
        }
    }
    sdl_canvas.present();
}

fn add_rows(row1: &[Vector3<f32>; WIDTH], row2: &[Vector3<f32>; WIDTH]) -> [Vector3<f32>; WIDTH] {
    let mut new_row = [Vector3 { x: 0.0, y: 0.0, z: 0.0}; WIDTH];
    
    for (i, (left, right)) in row1.iter().zip(row2).enumerate() {
        new_row[i] = left + right;
    }

    return new_row;
}

pub fn add_screens(screen1: Box<Screen>, screen2: Box<Screen>) -> Box<Screen> {

    let mut new_screen = [[Vector3 { x: 0.0, y: 0.0, z: 0.0}; WIDTH]; HEIGHT];

    let mut screen1_iter = screen1.iter();
    let mut screen2_iter = screen2.iter();

    let mut n_row = 0;
    loop {
        let left_item = screen1_iter.next();
        let right_item = screen2_iter.next();
        if left_item.is_none() || right_item.is_none() {
            break;
        }
        let left_row = left_item.unwrap();
        let right_row = right_item.unwrap();

        new_screen[n_row] = add_rows(left_row, right_row);
        n_row += 1;
    }

    return Box::new(new_screen);
}

use crate::scene::scene::Scene;
use crate::scene::screen::Screen;
use crate::scene::screen::{HEIGHT, WIDTH};
use cgmath::Vector3;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

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

    for x in 0..HEIGHT {
        for y in 0..WIDTH {
            // Calculate pixel color here.
            // DEBUG: Testing screen manipulation
            if y % 10 == 0 {
                new_screen[x][y] = Vector3 {
                    x: 255.0,
                    y: 0.0,
                    z: 255.0,
                };
            }
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

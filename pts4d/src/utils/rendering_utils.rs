use cgmath::Vector3;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::Canvas, video::Window,
};

use crate::scene::{
    camera::{Camera, CameraConfig},
    scene::Scene,
    screen::{Screen, HEIGHT, WIDTH},
};

pub fn preprocess_color(color: Vector3<f32>) -> Vector3<f32> {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    // Gamma transform;
    r = r.sqrt();
    b = b.sqrt();
    g = g.sqrt();

    r = r.clamp(0.0, 1.0);
    g = g.clamp(0.0, 1.0);
    b = b.clamp(0.0, 1.0);

    return Vector3::new(r * 255.0, g * 255.0, b * 255.0);
}

pub fn initialize_screen() -> Vec<Vec<Vector3<f32>>> {
    return vec![vec![Vector3::new(0.0, 0.0, 0.0); WIDTH]; HEIGHT];
}

pub fn add_screens(screen1: Screen, screen2: Screen) -> Screen {
    return screen1
        .iter()
        .zip(screen2.iter())
        .map(|(&ref row1, &ref row2)| add_rows(&row1, &row2))
        .collect();
}

#[inline]
fn add_rows(row1: &Vec<Vector3<f32>>, row2: &Vec<Vector3<f32>>) -> Vec<Vector3<f32>> {
    return row1.iter().zip(row2.iter()).map(|(&x, &y)| x + y).collect();
}

pub fn handle_input(event: Event, scene: &mut Scene) -> bool {
    match event {
        Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
        } => {
            scene.camera.camera_config.look_from.z -= 0.5;
            scene.camera.camera_config.look_at.z -= 0.5;
            scene.camera = renew_camera(&scene.camera.camera_config);
            return true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
        } => {
            scene.camera.camera_config.look_from.z += 0.5;
            scene.camera.camera_config.look_at.z += 0.5;
            scene.camera = renew_camera(&scene.camera.camera_config);
            return true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
        } => {
            scene.camera.camera_config.look_from.x += 0.5;
            scene.camera.camera_config.look_at.x += 0.5;

            scene.camera = renew_camera(&scene.camera.camera_config);
            return true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
        } => {
            scene.camera.camera_config.look_from.x -= 0.5;
            scene.camera.camera_config.look_at.x -= 0.5;
            scene.camera = renew_camera(&scene.camera.camera_config);
            return true;
        }
        Event::MouseWheel { precise_y, .. } => {
            scene.camera.camera_config.look_from.y += precise_y * 0.2;
            scene.camera.camera_config.look_at.y += precise_y * 0.2;

            scene.camera = renew_camera(&scene.camera.camera_config);
            return true;
        }
        _ => {
            return false;
        }
    }
}

pub fn renew_camera(config: &CameraConfig) -> Camera {
    return Camera::new(
        config.image_height,
        config.image_width,
        config.fov,
        config.look_from,
        config.look_at,
        config.up,
    );
}

pub fn present_screen(screen: &Screen, sdl_canvas: &mut Canvas<Window>, iteration: i32) {
    for (y, row) in screen.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let preprocessed_pixel = preprocess_color(*pixel / iteration as f32);
            let color = Color {
                r: (preprocessed_pixel.x) as u8,
                g: (preprocessed_pixel.y) as u8,
                b: (preprocessed_pixel.z) as u8,
                a: 0,
            };
            sdl_canvas.set_draw_color(color);
            sdl_canvas
                .draw_point(Point::new(x as i32, y as i32))
                .unwrap();
        }
    }
    sdl_canvas.present();
}

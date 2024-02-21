extern crate sdl2;

mod object {
    pub mod object;
}

mod materials {
    pub mod material;
}

mod scene {
    pub mod camera;
    pub mod scene;
    pub mod screen;
}

mod utils {
    pub mod rendering_utils;
    pub mod scene_builders;
    pub mod vector_utils;
}

mod renderer;

use crate::scene::scene::Scene;
use crate::scene::screen::{HEIGHT, WIDTH};
use crate::utils::rendering_utils::{add_screens, handle_input, initialize_screen, present_screen};
use crate::utils::scene_builders;

use renderer::render_pass;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Instant;

pub fn main() -> Result<(), String> {
    println!("Welcome to PTS4D!");

    // SDL Boilerplate
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("PTS4D", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // Set default background color to black.
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();

    // Paint the screen
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    // Initialize PTS4D World
    // let scene: Scene = scene_builders::generate_scene();

    // scene is mutable, so the camera can be modified during runtime.
    let mut scene: Scene = scene_builders::generate_polygon_scene("./objs/chill/cube.obj");

    // Keep track of iterations
    let mut curr_samples_per_pixel = 0;

    // Keeps the sum of all colors across all iterations
    let mut all_frames = initialize_screen();
    'running: loop {
        let start_time = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    if handle_input(event, &mut scene) {
                        // If the camera has changed something,
                        // Delete the frame and start rendering a new one.
                        all_frames = initialize_screen();
                        curr_samples_per_pixel = 0; // Set the current samples per pixel to 0;
                    };
                }
            }
        }
        curr_samples_per_pixel += 1;
        all_frames = add_screens(all_frames, render_pass(&scene));
        present_screen(&all_frames, &mut canvas, curr_samples_per_pixel);

        let end_time = Instant::now() - start_time;
        println!(
            "Frame {} in {:?} - {} FPS",
            curr_samples_per_pixel,
            end_time,
            1000.0 / end_time.as_millis() as f32
        );
    }

    println!("Bye!");

    return Ok(());
}

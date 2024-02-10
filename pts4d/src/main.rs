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
    pub mod scene_builders;
    pub mod vector_utils;
}

mod renderer;

use crate::scene::scene::Scene;
use crate::scene::screen::{self, Screen};
use crate::utils::scene_builders;

use renderer::{present_screen, render_pass};
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
        .window("PTS4D", screen::WIDTH as u32, screen::HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // Set default background color to black.
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Paint the screen
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    // Initialize PTS4D World
    let scene: Scene = scene_builders::generate_scene();

    // Keep track of iterations
    let mut i = 0;
    let mut last_frame: Option<Box<Screen>> = None;

    'running: loop {
        let start_time = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let current_frame = Some(render_pass(&scene, last_frame));
        last_frame = current_frame.clone();
        present_screen(current_frame.unwrap(), &mut canvas);

        i += 1;
        let end_time = (Instant::now() - start_time).as_millis();
        println!(
            "Frame {} in {:?}ms - {} FPS",
            i,
            end_time,
            1000.0 / end_time as f32
        );
    }

    println!("Bye!");

    return Ok(());
}

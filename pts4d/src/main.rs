extern crate sdl2;

mod utils {
    pub mod screen;
}

mod renderer;

use crate::utils::screen;
use renderer::render_pass;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    // Generic SDL window management logic
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    println!("Welcome to PTS4D!");

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
    let mut t = 0;

    let mut rendering_screen: screen::Screen =
        [[Color::RGB(0, 0, 0); screen::WIDTH]; screen::HEIGHT];

    'running: loop {
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

        t += 1;

        // Prevent overflows
        if t < screen::WIDTH {
            rendering_screen[50][t] = Color::RGB(255, 255, 255);
        }

        // Magic happens here
        render_pass(&mut rendering_screen, &mut canvas).unwrap();

        println!("Iteration {}", t);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    println!("Bye!");
    Ok(())
}

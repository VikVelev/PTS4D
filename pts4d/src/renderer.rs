use crate::scene::camera::Camera;
use crate::scene::scene::Scene;
use crate::scene::screen::Screen;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn render_pass(
    scene: &Scene,
    screen: &mut Screen,
    canvas: &mut Canvas<Window>,
) -> Result<(), String> {
    for (y, row) in screen.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            // Calculate pixel color here.
            canvas.set_draw_color(*pixel);
            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
        }
    }
    canvas.present();
    Ok(())
}

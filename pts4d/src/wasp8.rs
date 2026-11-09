// Renders a Wasp8 (CHIP-8) framebuffer as a path traced scene.
//
// Rendering happens on its own thread. The emulator pushes new grids with
// submit_grid() and picks up the newest finished image with with_new_image(),
// neither of them waits for a render pass to finish.
//
// While the grid stays the same samples keep piling up and the image converges.
// When it changes we keep a bit of the old image around (kinda like phosphor on
// an old CRT) so moving stuff isn't just single sample noise.

use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

use cgmath::{InnerSpace, Vector3};
use rayon::prelude::*;

use crate::materials::material::Material;
use crate::object::pixel_grid::PixelGrid;
use crate::object::sphere::Sphere;
use crate::renderer::ray_trace_with_depth;
use crate::scene::camera::Camera;
use crate::scene::scene::Scene;
use crate::utils::rendering_utils::preprocess_color;

// Stop rendering once a still frame has this many samples per pixel.
const MAX_ACCUMULATED_SPP: u32 = 4096;
// How many samples per pixel of the old image survive a grid change.
const PERSISTENCE_SPP: u32 = 3;

const CELL_SIZE: f32 = 0.1;
// How far each cube sticks out, in cells (1.0 = actual cubes).
const CUBE_DEPTH: f32 = 3.0;
// Gap around each cube as a fraction of a cell, 0 = neighbours fuse together.
const CUBE_GAP: f32 = 0.0;

// Brightness of the front face of a lit pixel.
const FRONT_GLOW: f32 = 1.6;
// Brightness of the light that shades the sides of the cubes.
const KEY_LIGHT: f32 = 8.0;

// Camera orbits around the middle of the grid. 0 / 0 looks straight at it.
const CAMERA_YAW_DEG: f32 = 0.0;
const CAMERA_PITCH_DEG: f32 = 32.0;
// Narrower FoV = flatter perspective.
const CAMERA_FOV_DEG: f32 = 28.0;
// Space around the grid, 1.0 = edge to edge.
const FRAME_MARGIN: f32 = 1.08;

#[derive(Clone, Copy, Debug)]
pub struct Settings {
    pub out_width: usize,
    pub out_height: usize,
    pub grid_width: usize,
    pub grid_height: usize,
    pub samples_per_frame: u32,
    pub max_bounces: u32,
    pub threads: usize,
    pub fg_rgb: u32,
    pub bg_rgb: u32,
}

pub struct Image<'a> {
    pub pixels: &'a [u32], // 0xAARRGGBB
    pub spp: u32,
}

fn rgb_to_vec(rgb: u32) -> Vector3<f32> {
    let channel = |shift: u32| ((rgb >> shift) & 0xFF) as f32 / 255.0;
    // Undo the sqrt gamma from preprocess_color, otherwise colors come out washed.
    let linear = |v: f32| v * v;
    return Vector3::new(linear(channel(16)), linear(channel(8)), linear(channel(0)));
}

// Every lit pixel becomes a cube with a glowing front and diffuse sides, sitting
// above a glossy floor in front of a wall tinted with the background color.
pub fn build_scene(settings: &Settings, grid: &[u8], fg_rgb: u32, bg_rgb: u32) -> Scene {
    let grid_w = settings.grid_width as f32 * CELL_SIZE;
    let grid_h = settings.grid_height as f32 * CELL_SIZE;
    let floor_y = 0.0;
    let grid_center = Vector3::new(0.0, floor_y + 0.25 + grid_h / 2.0, 0.0);

    // Since up is -Y, the camera maps image-right to world -X,
    // so flip the columns or everything ends up mirrored.
    let mirrored: Vec<u8> = grid
        .chunks(settings.grid_width)
        .flat_map(|row| row.iter().rev().copied())
        .collect();

    let pixels = PixelGrid::new(
        settings.grid_width,
        settings.grid_height,
        &mirrored,
        grid_center,
        CELL_SIZE,
        CUBE_DEPTH * CELL_SIZE,
        CUBE_GAP,
        Material::Emissive(rgb_to_vec(fg_rgb), FRONT_GLOW),
        Material::Diffuse(rgb_to_vec(fg_rgb) * 0.85),
    );

    let floor = Sphere::new(
        Vector3::new(0.0, floor_y - 500.0, 0.0),
        500.0,
        Material::Metallic(Vector3::new(0.55, 0.55, 0.6), 0.25),
    );

    let back_wall = Sphere::new(
        Vector3::new(0.0, 0.0, -(CUBE_DEPTH * CELL_SIZE / 2.0 + 0.5) - 500.0),
        500.0,
        Material::Diffuse(rgb_to_vec(bg_rgb) * 0.8 + Vector3::new(0.05, 0.05, 0.05)),
    );

    // Big light up and to the left (world -X is image-left)
    let key_light = Sphere::new(
        grid_center + Vector3::new(-grid_w * 0.7, grid_h * 2.2, grid_w * 0.6),
        grid_w * 0.4,
        Material::Emissive(Vector3::new(1.0, 0.97, 0.92), KEY_LIGHT),
    );

    let aspect = settings.out_width as f32 / settings.out_height as f32;
    let half_tan = (CAMERA_FOV_DEG.to_radians() / 2.0).tan();
    let yaw = CAMERA_YAW_DEG.to_radians();
    let pitch = CAMERA_PITCH_DEG.to_radians();
    let direction = Vector3::new(
        -yaw.sin() * pitch.cos(),
        pitch.sin(),
        yaw.cos() * pitch.cos(),
    );

    let (look_at, distance) = frame_grid(direction, grid_center, grid_w, grid_h, half_tan, aspect);
    let look_from = look_at + distance * direction;
    let up = Vector3::new(0.0, -1.0, 0.0); // TODO: WTF? (same as in scene_builders)
    let camera = Camera::new(
        settings.out_height as f32,
        settings.out_width as f32,
        CAMERA_FOV_DEG,
        look_from,
        look_at,
        up,
    );

    let mut scene = Scene::build_complex_scene(vec![], vec![floor, back_wall, key_light], camera);
    scene.grids.push(pixels);
    return scene;
}

// Finds where to look and how far back the camera has to be so the whole grid
// fits on screen and is centered. Because of perspective the closer edge is
// bigger, so this projects the actual corners instead of using the flat size.
fn frame_grid(
    direction: Vector3<f32>,
    grid_center: Vector3<f32>,
    grid_w: f32,
    grid_h: f32,
    half_tan: f32,
    aspect: f32,
) -> (Vector3<f32>, f32) {
    let w = direction.normalize();
    let u = Vector3::new(0.0, 1.0, 0.0).cross(w).normalize();
    let v = w.cross(u);
    let limit_x = half_tan * aspect / FRAME_MARGIN;
    let limit_y = half_tan / FRAME_MARGIN;

    let mut corners = Vec::new();
    for sx in [-0.5, 0.5] {
        for sy in [-0.5, 0.5] {
            for sz in [-0.5, 0.5] {
                let offset = Vector3::new(sx * grid_w, sy * grid_h, sz * CUBE_DEPTH * CELL_SIZE);
                corners.push(grid_center + offset);
            }
        }
    }

    // Screen space bounds (min x, max x, min y, max y) of the corners,
    // None if a corner ends up behind the camera.
    let project = |look_at: Vector3<f32>, distance: f32| -> Option<(f32, f32, f32, f32)> {
        let eye = look_at + distance * w;
        let mut bounds = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for corner in &corners {
            let rel = corner - eye;
            let z = -rel.dot(w);
            if z <= 0.0 {
                return None;
            }
            let x = rel.dot(u) / z;
            let y = rel.dot(v) / z;
            bounds = (
                bounds.0.min(x),
                bounds.1.max(x),
                bounds.2.min(y),
                bounds.3.max(y),
            );
        }
        return Some(bounds);
    };

    let mut look_at = grid_center;
    let mut distance = 1.0;

    for _ in 0..6 {
        // Binary search the closest distance where everything still fits...
        let fits = |d: f32| match project(look_at, d) {
            Some((x0, x1, y0, y1)) => {
                x0.abs().max(x1.abs()) <= limit_x && y0.abs().max(y1.abs()) <= limit_y
            }
            None => false,
        };

        let mut near = 0.01;
        let mut far = 1000.0;
        for _ in 0..50 {
            let mid = 0.5 * (near + far);
            if fits(mid) {
                far = mid;
            } else {
                near = mid;
            }
        }
        distance = far;

        // ...then nudge the look at point so the grid ends up in the middle.
        let (x0, x1, y0, y1) = project(look_at, distance).unwrap();
        look_at += u * ((x0 + x1) / 2.0 * distance) + v * ((y0 + y1) / 2.0 * distance);
    }

    return (look_at, distance);
}

struct Input {
    grid: Vec<u8>,
    fg_rgb: u32,
    bg_rgb: u32,
    generation: u64, // bumped on every change
    shutdown: bool,
}

struct Output {
    pixels: Vec<u32>,
    spp: u32,
    seq: u64, // bumped on every finished pass
    failure: Option<String>,
}

struct Shared {
    input: Mutex<Input>,
    wake: Condvar,
    output: Mutex<Output>,
}

pub struct Renderer {
    settings: Settings,
    shared: Arc<Shared>,
    thread: Option<JoinHandle<()>>,
    last_seq: u64,
}

impl Renderer {
    pub fn new(settings: Settings) -> Result<Renderer, String> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(settings.threads)
            .thread_name(|i| format!("pts4d-worker-{i}"))
            .build()
            .map_err(|e| format!("failed to start render threads: {e}"))?;

        let shared = Arc::new(Shared {
            input: Mutex::new(Input {
                grid: vec![0; settings.grid_width * settings.grid_height],
                fg_rgb: settings.fg_rgb,
                bg_rgb: settings.bg_rgb,
                generation: 0,
                shutdown: false,
            }),
            wake: Condvar::new(),
            output: Mutex::new(Output {
                pixels: vec![0xFF00_0000; settings.out_width * settings.out_height],
                spp: 0,
                seq: 0,
                failure: None,
            }),
        });

        let thread_shared = Arc::clone(&shared);
        let thread = std::thread::Builder::new()
            .name("pts4d-render".into())
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    render_loop(settings, &thread_shared, &pool)
                }));

                // If the render thread dies, tell whoever asks for the next frame.
                if let Err(e) = result {
                    let msg = e
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                        .unwrap_or_else(|| "unknown panic".into());

                    if let Ok(mut out) = thread_shared.output.lock() {
                        out.failure = Some(format!("render thread panicked: {msg}"));
                    }
                }
            })
            .map_err(|e| format!("failed to spawn render thread: {e}"))?;

        return Ok(Renderer {
            settings,
            shared,
            thread: Some(thread),
            last_seq: 0,
        });
    }

    pub fn settings(&self) -> &Settings {
        return &self.settings;
    }

    // Only restarts the accumulation if something actually changed.
    pub fn submit_grid(&mut self, cells: &[u8]) {
        let mut input = self.shared.input.lock().unwrap();
        let changed = input
            .grid
            .iter()
            .zip(cells)
            .any(|(&a, &b)| (a != 0) != (b != 0));

        if changed {
            input.grid.copy_from_slice(cells);
            input.generation += 1;
            self.shared.wake.notify_one();
        }
    }

    pub fn set_palette(&mut self, fg_rgb: u32, bg_rgb: u32) {
        let mut input = self.shared.input.lock().unwrap();
        if input.fg_rgb != fg_rgb || input.bg_rgb != bg_rgb {
            input.fg_rgb = fg_rgb;
            input.bg_rgb = bg_rgb;
            input.generation += 1;
            self.shared.wake.notify_one();
        }
    }

    // Calls f with the newest image if a new one finished since the last call.
    // Ok(None) = nothing new, Err = the render thread died.
    pub fn with_new_image<R>(&mut self, f: impl FnOnce(Image) -> R) -> Result<Option<R>, String> {
        let out = self.shared.output.lock().unwrap();

        if let Some(failure) = &out.failure {
            return Err(failure.clone());
        }

        if out.seq == self.last_seq {
            return Ok(None);
        }

        self.last_seq = out.seq;
        return Ok(Some(f(Image {
            pixels: &out.pixels,
            spp: out.spp,
        })));
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        if let Ok(mut input) = self.shared.input.lock() {
            input.shutdown = true;
        }
        self.shared.wake.notify_all();

        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn render_loop(settings: Settings, shared: &Shared, pool: &rayon::ThreadPool) {
    let (width, height) = (settings.out_width, settings.out_height);
    let samples = settings.samples_per_frame;
    let max_depth = settings.max_bounces as i32;

    // Sum of all passes so far, the current pass, and the final colors
    let mut all_frames = vec![Vector3::new(0.0, 0.0, 0.0); width * height];
    let mut frame = vec![Vector3::new(0.0, 0.0, 0.0); width * height];
    let mut colors = vec![0u32; width * height];

    let mut spp: u32 = 0;
    let mut generation = u64::MAX;
    let mut palette: Option<(u32, u32)> = None;
    let mut scene: Option<Scene> = None;

    loop {
        {
            let mut input = shared.input.lock().unwrap();

            // Nothing new and the image is done, so sleep until something changes.
            while !input.shutdown && input.generation == generation && spp >= MAX_ACCUMULATED_SPP {
                input = shared.wake.wait(input).unwrap();
            }

            if input.shutdown {
                return;
            }

            if input.generation != generation {
                generation = input.generation;
                scene = Some(build_scene(
                    &settings,
                    &input.grid,
                    input.fg_rgb,
                    input.bg_rgb,
                ));

                let new_palette = Some((input.fg_rgb, input.bg_rgb));
                if palette != new_palette {
                    // New colors, start over.
                    palette = new_palette;
                    all_frames
                        .iter_mut()
                        .for_each(|c| *c = Vector3::new(0.0, 0.0, 0.0));
                    spp = 0;
                } else if spp > PERSISTENCE_SPP {
                    // Grid changed, fade the old image down to a few samples.
                    let keep = PERSISTENCE_SPP as f32 / spp as f32;
                    pool.install(|| all_frames.par_iter_mut().for_each(|c| *c *= keep));
                    spp = PERSISTENCE_SPP;
                }
            }
        }

        let scene = scene.as_ref().unwrap();

        pool.install(|| {
            frame
                .par_chunks_mut(width)
                .enumerate()
                .for_each(|(y, row)| {
                    for (x, pixel) in row.iter_mut().enumerate() {
                        let mut color = Vector3::new(0.0, 0.0, 0.0);
                        for _ in 0..samples {
                            let ray =
                                scene.shoot_ray(x as f32 / width as f32, y as f32 / height as f32);
                            color += ray_trace_with_depth(scene, &ray, max_depth);
                        }
                        *pixel = color;
                    }
                });
        });

        // Always publish the pass, even if the grid changed meanwhile.
        // Throwing it away made the picture stall whenever the game was busy.
        spp += samples;
        let inv_spp = 1.0 / spp as f32;
        pool.install(|| {
            all_frames
                .par_iter_mut()
                .zip(frame.par_iter())
                .zip(colors.par_iter_mut())
                .for_each(|((sum, new), out)| {
                    *sum += *new;
                    let c = preprocess_color(*sum * inv_spp);
                    *out = 0xFF00_0000 | ((c.x as u32) << 16) | ((c.y as u32) << 8) | (c.z as u32);
                });
        });

        let mut out = shared.output.lock().unwrap();
        std::mem::swap(&mut out.pixels, &mut colors);
        out.spp = spp;
        out.seq += 1;
    }
}

// C ABI so Wasp8 (or anything else that speaks C) can use the wasp8 renderer.
// Header lives in include/pts4d.h, keep both in sync and bump the version
// if anything breaks.
//
// Every function catches panics so nothing ever unwinds into C.

use std::cell::RefCell;
use std::ffi::{c_char, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::wasp8::{Renderer, Settings};

pub const PTS4D_ABI_VERSION: u32 = 1;

const MAX_OUTPUT_SIZE: u32 = 16384;
const MAX_GRID_SIZE: u32 = 1024;
const MAX_BOUNCES: u32 = 64;
const DEFAULT_BOUNCES: u32 = 4;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pts4dStatus {
    Ok = 0,
    NoNewFrame = 1,
    ErrNull = -1,
    ErrArg = -2,
    ErrVersion = -3,
    ErrPanic = -4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Pts4dConfig {
    pub struct_size: u32,
    pub abi_version: u32,
    pub out_width: u32,
    pub out_height: u32,
    pub grid_width: u32,
    pub grid_height: u32,
    pub samples_per_frame: u32,
    pub max_bounces: u32,
    pub threads: u32,
    pub fg_rgb: u32,
    pub bg_rgb: u32,
}

// What C gets back as an opaque pointer
pub struct Pts4dRenderer {
    inner: Renderer,
    last_error: CString,
}

thread_local! {
    // For errors where there's no handle to put them on (e.g. create failing)
    static LAST_ERROR: RefCell<CString> = RefCell::new(CString::default());
}

fn to_cstring(msg: impl Into<String>) -> CString {
    return CString::new(msg.into().replace('\0', " ")).unwrap_or_default();
}

fn set_last_error(msg: impl Into<String>) {
    let msg = to_cstring(msg);
    LAST_ERROR.with(|e| *e.borrow_mut() = msg);
}

fn panic_message(e: Box<dyn std::any::Any + Send>) -> String {
    return e
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown panic".into());
}

impl Pts4dRenderer {
    fn fail(&mut self, status: Pts4dStatus, msg: impl Into<String>) -> Pts4dStatus {
        self.last_error = to_cstring(msg);
        return status;
    }
}

// Checks the handle and runs f, a panic turns into PTS4D_ERR_PANIC.
unsafe fn with_handle(
    r: *mut Pts4dRenderer,
    f: impl FnOnce(&mut Pts4dRenderer) -> Pts4dStatus,
) -> Pts4dStatus {
    let Some(handle) = r.as_mut() else {
        set_last_error("renderer handle is NULL");
        return Pts4dStatus::ErrNull;
    };

    return match catch_unwind(AssertUnwindSafe(|| f(&mut *handle))) {
        Ok(status) => status,
        Err(e) => {
            handle.last_error = to_cstring(format!("panic: {}", panic_message(e)));
            Pts4dStatus::ErrPanic
        }
    };
}

fn validate(config: &Pts4dConfig) -> Result<Settings, (Pts4dStatus, String)> {
    if config.abi_version != PTS4D_ABI_VERSION {
        return Err((
            Pts4dStatus::ErrVersion,
            format!(
                "ABI version {} doesn't match the library's {}",
                config.abi_version, PTS4D_ABI_VERSION
            ),
        ));
    }

    let expected_size = std::mem::size_of::<Pts4dConfig>();
    if config.struct_size as usize != expected_size {
        return Err((
            Pts4dStatus::ErrVersion,
            format!(
                "Pts4dConfig is {} bytes, expected {}",
                config.struct_size, expected_size
            ),
        ));
    }

    let in_range = |v: u32, max: u32| v > 0 && v <= max;

    if !in_range(config.out_width, MAX_OUTPUT_SIZE) || !in_range(config.out_height, MAX_OUTPUT_SIZE)
    {
        return Err((
            Pts4dStatus::ErrArg,
            format!("bad output size {}x{}", config.out_width, config.out_height),
        ));
    }

    if !in_range(config.grid_width, MAX_GRID_SIZE) || !in_range(config.grid_height, MAX_GRID_SIZE) {
        return Err((
            Pts4dStatus::ErrArg,
            format!("bad grid size {}x{}", config.grid_width, config.grid_height),
        ));
    }

    let max_bounces = match config.max_bounces {
        0 => DEFAULT_BOUNCES,
        bounces => bounces.min(MAX_BOUNCES),
    };

    return Ok(Settings {
        out_width: config.out_width as usize,
        out_height: config.out_height as usize,
        grid_width: config.grid_width as usize,
        grid_height: config.grid_height as usize,
        samples_per_frame: config.samples_per_frame.max(1),
        max_bounces,
        threads: config.threads as usize,
        fg_rgb: config.fg_rgb & 0xFFFFFF,
        bg_rgb: config.bg_rgb & 0xFFFFFF,
    });
}

#[no_mangle]
pub unsafe extern "C" fn pts4d_create(
    config: *const Pts4dConfig,
    out: *mut *mut Pts4dRenderer,
) -> Pts4dStatus {
    let result = catch_unwind(AssertUnwindSafe(|| {
        if config.is_null() || out.is_null() {
            set_last_error("config or out is NULL");
            return Pts4dStatus::ErrNull;
        }
        *out = std::ptr::null_mut();

        let settings = match validate(&*config) {
            Ok(settings) => settings,
            Err((status, msg)) => {
                set_last_error(msg);
                return status;
            }
        };

        return match Renderer::new(settings) {
            Ok(inner) => {
                let handle = Box::new(Pts4dRenderer {
                    inner,
                    last_error: CString::default(),
                });
                *out = Box::into_raw(handle);
                Pts4dStatus::Ok
            }
            Err(msg) => {
                set_last_error(msg);
                Pts4dStatus::ErrArg
            }
        };
    }));

    return result.unwrap_or_else(|e| {
        set_last_error(format!("panic: {}", panic_message(e)));
        Pts4dStatus::ErrPanic
    });
}

// Stops the render thread and frees the handle, NULL is fine.
#[no_mangle]
pub unsafe extern "C" fn pts4d_destroy(r: *mut Pts4dRenderer) {
    if r.is_null() {
        return;
    }
    let _ = catch_unwind(AssertUnwindSafe(|| drop(Box::from_raw(r))));
}

#[no_mangle]
pub unsafe extern "C" fn pts4d_submit_grid(
    r: *mut Pts4dRenderer,
    cells: *const u8,
    len: usize,
) -> Pts4dStatus {
    return with_handle(r, |handle| {
        if cells.is_null() {
            return handle.fail(Pts4dStatus::ErrNull, "cells is NULL");
        }

        let settings = handle.inner.settings();
        let expected = settings.grid_width * settings.grid_height;
        if len != expected {
            return handle.fail(
                Pts4dStatus::ErrArg,
                format!("grid is {len} bytes, expected {expected}"),
            );
        }

        handle
            .inner
            .submit_grid(std::slice::from_raw_parts(cells, len));
        return Pts4dStatus::Ok;
    });
}

#[no_mangle]
pub unsafe extern "C" fn pts4d_acquire_image(
    r: *mut Pts4dRenderer,
    argb8888: *mut u32,
    stride_bytes: usize,
    out_spp: *mut u32,
) -> Pts4dStatus {
    return with_handle(r, |handle| {
        if argb8888.is_null() {
            return handle.fail(Pts4dStatus::ErrNull, "argb8888 is NULL");
        }

        let width = handle.inner.settings().out_width;
        let height = handle.inner.settings().out_height;
        if stride_bytes < width * 4 || stride_bytes % 4 != 0 {
            return handle.fail(
                Pts4dStatus::ErrArg,
                format!("stride {stride_bytes} doesn't work for width {width}"),
            );
        }
        let stride = stride_bytes / 4;

        let result = handle.inner.with_new_image(|image| {
            for y in 0..height {
                let src = &image.pixels[y * width..(y + 1) * width];
                let dst = std::slice::from_raw_parts_mut(argb8888.add(y * stride), width);
                dst.copy_from_slice(src);
            }
            return image.spp;
        });

        return match result {
            Ok(Some(spp)) => {
                if !out_spp.is_null() {
                    *out_spp = spp;
                }
                Pts4dStatus::Ok
            }
            Ok(None) => Pts4dStatus::NoNewFrame,
            Err(msg) => handle.fail(Pts4dStatus::ErrPanic, msg),
        };
    });
}

#[no_mangle]
pub unsafe extern "C" fn pts4d_set_palette(
    r: *mut Pts4dRenderer,
    fg_rgb: u32,
    bg_rgb: u32,
) -> Pts4dStatus {
    return with_handle(r, |handle| {
        handle
            .inner
            .set_palette(fg_rgb & 0xFFFFFF, bg_rgb & 0xFFFFFF);
        return Pts4dStatus::Ok;
    });
}

// Valid until the next call on the same handle (or the same thread for NULL)
#[no_mangle]
pub unsafe extern "C" fn pts4d_last_error(r: *const Pts4dRenderer) -> *const c_char {
    return match r.as_ref() {
        Some(handle) => handle.last_error.as_ptr(),
        None => LAST_ERROR.with(|e| e.borrow().as_ptr()),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn test_config(width: u32, height: u32) -> Pts4dConfig {
        return Pts4dConfig {
            struct_size: std::mem::size_of::<Pts4dConfig>() as u32,
            abi_version: PTS4D_ABI_VERSION,
            out_width: width,
            out_height: height,
            grid_width: 64,
            grid_height: 32,
            samples_per_frame: 1,
            max_bounces: 3,
            threads: 2,
            fg_rgb: 0xF4F4F4,
            bg_rgb: 0x101B2D,
        };
    }

    #[test]
    fn config_matches_header() {
        // 11 x uint32_t in pts4d.h
        assert_eq!(std::mem::size_of::<Pts4dConfig>(), 44);
        assert_eq!(std::mem::align_of::<Pts4dConfig>(), 4);
    }

    #[test]
    fn rejects_bad_input() {
        unsafe {
            let mut r = std::ptr::null_mut();

            let mut config = test_config(64, 32);
            config.abi_version = 99;
            assert_eq!(pts4d_create(&config, &mut r), Pts4dStatus::ErrVersion);
            assert!(r.is_null());

            config = test_config(0, 32);
            assert_eq!(pts4d_create(&config, &mut r), Pts4dStatus::ErrArg);

            assert_eq!(pts4d_create(std::ptr::null(), &mut r), Pts4dStatus::ErrNull);
            assert_eq!(
                pts4d_submit_grid(std::ptr::null_mut(), [0u8].as_ptr(), 1),
                Pts4dStatus::ErrNull
            );

            pts4d_destroy(std::ptr::null_mut());
        }
    }

    #[test]
    fn renders_and_accumulates() {
        unsafe {
            let (width, height) = (128u32, 64u32);
            let mut r = std::ptr::null_mut();
            assert_eq!(
                pts4d_create(&test_config(width, height), &mut r),
                Pts4dStatus::Ok
            );

            // Lit block in the middle of the screen
            let mut grid = vec![0u8; 64 * 32];
            for x in 20..44 {
                for y in 10..22 {
                    grid[y * 64 + x] = 1;
                }
            }
            assert_eq!(pts4d_submit_grid(r, grid.as_ptr(), 10), Pts4dStatus::ErrArg);
            assert_eq!(
                pts4d_submit_grid(r, grid.as_ptr(), grid.len()),
                Pts4dStatus::Ok
            );

            // Padded rows, to check the stride is respected
            let stride = (width as usize + 8) * 4;
            let mut pixels = vec![0u32; stride / 4 * height as usize];
            let mut spp = 0;
            let deadline = Instant::now() + Duration::from_secs(20);

            while spp < 8 && Instant::now() < deadline {
                match pts4d_acquire_image(r, pixels.as_mut_ptr(), stride, &mut spp) {
                    Pts4dStatus::Ok | Pts4dStatus::NoNewFrame => {}
                    status => panic!("acquire failed: {status:?}"),
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            assert!(spp >= 8, "stuck at {spp} spp");

            let brightness = |p: u32| ((p >> 16) & 0xFF) + ((p >> 8) & 0xFF) + (p & 0xFF);
            let center = pixels[(height as usize / 2) * (stride / 4) + width as usize / 2];
            let corner = pixels[0];
            assert!(
                brightness(center) > brightness(corner) + 200,
                "center {center:08x} corner {corner:08x}"
            );

            // Padding is never touched
            assert_eq!(pixels[width as usize], 0);

            pts4d_destroy(r);
        }
    }
}

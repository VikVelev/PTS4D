pub mod object {
    pub mod mesh;
    pub mod object;
    pub mod pixel_grid;
    pub mod sphere;
}

pub mod materials {
    pub mod material;
}

pub mod scene {
    pub mod camera;
    pub mod scene;
    pub mod screen;
}

pub mod accel {
    pub mod aabb;
    pub mod bvh;
}

pub mod utils {
    pub mod rendering_utils;
    pub mod scene_builders;
    pub mod vector_utils;
}

pub mod renderer;

// Renders the Wasp8 (CHIP-8) screen
pub mod wasp8;

// C ABI for the above, see include/pts4d.h
pub mod ffi;

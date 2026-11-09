use cgmath::Vector3;

use crate::{
    accel::aabb::AABB,
    materials::material::Material,
    utils::vector_utils::{Interval, Ray},
};

use super::object::{Hit, Hitable};

// A framebuffer (e.g. the 64x32 CHIP-8 screen) as a wall of boxes.
//
// The grid stands up in the XY plane, row 0 is the top row like on the CHIP-8.
// Every lit cell is a box `depth` deep, unlit cells are just empty space.
// The face looking at +Z (towards the camera) uses `front`, the other five use
// `body`, so the light can actually show the sides.
//
// Instead of testing every box we walk the cells the ray passes over (2D DDA),
// so the cost depends on how many cells the ray crosses, not how many are lit.
pub struct PixelGrid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    min: Vector3<f32>, // corner at column 0, bottom row, back side
    cell_size: f32,
    depth: f32,
    inset: f32,
    front: Material,
    body: Material,
    bbox: AABB,
}

impl PixelGrid {
    // `cells` goes row by row starting at the top, anything non-zero is lit.
    // `gap` is how much of a cell to leave empty on each side, 0 = fused boxes.
    pub fn new(
        width: usize,
        height: usize,
        cells: &[u8],
        center: Vector3<f32>,
        cell_size: f32,
        depth: f32,
        gap: f32,
        front: Material,
        body: Material,
    ) -> PixelGrid {
        assert_eq!(cells.len(), width * height, "grid size mismatch");

        let half_size = Vector3::new(
            width as f32 * cell_size / 2.0,
            height as f32 * cell_size / 2.0,
            depth / 2.0,
        );
        let min = center - half_size;
        let max = center + half_size;

        return PixelGrid {
            width,
            height,
            cells: cells.iter().map(|&c| c != 0).collect(),
            min,
            cell_size,
            depth,
            inset: cell_size * gap.clamp(0.0, 0.45),
            front,
            body,
            bbox: AABB::new_from_diagonals(min, max),
        };
    }

    // i = column, j = row counted from the bottom
    #[inline]
    fn is_lit(&self, i: usize, j: usize) -> bool {
        let row = self.height - 1 - j;
        return self.cells[row * self.width + i];
    }

    // Where the ray enters and leaves the box between `lo` and `hi`,
    // plus the axis it entered through (needed for the normal).
    #[inline]
    fn slab(ray: &Ray, lo: Vector3<f32>, hi: Vector3<f32>) -> Option<(f32, f32, usize)> {
        let mut t_near = f32::NEG_INFINITY;
        let mut t_far = f32::INFINITY;
        let mut axis = 0;

        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (lo[a] - ray.origin[a]) * inv_d;
            let mut t1 = (hi[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > t_near {
                t_near = t0;
                axis = a;
            }

            if t1 < t_far {
                t_far = t1;
            }

            if t_far < t_near {
                return None;
            }
        }

        return Some((t_near, t_far, axis));
    }

    fn intersect_cell(&self, ray: &Ray, i: usize, j: usize, bounds: &Interval) -> Option<Hit> {
        let c = self.cell_size;
        let lo = self.min + Vector3::new(i as f32 * c + self.inset, j as f32 * c + self.inset, 0.0);
        let hi = self.min
            + Vector3::new(
                (i + 1) as f32 * c - self.inset,
                (j + 1) as f32 * c - self.inset,
                self.depth,
            );

        let (t, _, axis) = PixelGrid::slab(ray, lo, hi)?;
        if t <= bounds.min || t >= bounds.max {
            return None;
        }

        let mut normal = Vector3::new(0.0, 0.0, 0.0);
        normal[axis] = if ray.direction[axis] > 0.0 { -1.0 } else { 1.0 };

        let material = if axis == 2 && normal.z > 0.0 {
            &self.front
        } else {
            &self.body
        };

        return Some(Hit {
            point_at_intersection: t,
            point: ray.point_at(t),
            normal,
            is_facing_you: true,
            material,
        });
    }
}

impl Hitable for PixelGrid {
    fn intersect(&self, ray: &Ray, bounds: Interval) -> Option<Hit> {
        let max = self.min
            + Vector3::new(
                self.width as f32 * self.cell_size,
                self.height as f32 * self.cell_size,
                self.depth,
            );

        let (t_enter, t_exit, _) = PixelGrid::slab(ray, self.min, max)?;
        let t_start = t_enter.max(bounds.min);
        let t_end = t_exit.min(bounds.max);
        if t_start >= t_end {
            return None;
        }

        // Start in the cell where the ray enters the grid
        let c = self.cell_size;
        let start = ray.point_at(t_start);
        let to_cell = |v: f32, n: usize| (v.floor().max(0.0) as usize).min(n - 1);
        let mut i = to_cell((start.x - self.min.x) / c, self.width);
        let mut j = to_cell((start.y - self.min.y) / c, self.height);

        let dx = ray.direction.x;
        let dy = ray.direction.y;
        let step_i: isize = if dx > 0.0 { 1 } else { -1 };
        let step_j: isize = if dy > 0.0 { 1 } else { -1 };

        // t at which the ray crosses into the next column / row
        let next_edge = |idx: usize, step: isize, origin: f32, min: f32, d: f32| -> f32 {
            if d == 0.0 {
                return f32::INFINITY;
            }
            let edge = min + (idx as f32 + if step > 0 { 1.0 } else { 0.0 }) * c;
            return (edge - origin) / d;
        };
        let mut t_next_x = next_edge(i, step_i, ray.origin.x, self.min.x, dx);
        let mut t_next_y = next_edge(j, step_j, ray.origin.y, self.min.y, dy);
        let t_step_x = if dx == 0.0 {
            f32::INFINITY
        } else {
            c / dx.abs()
        };
        let t_step_y = if dy == 0.0 {
            f32::INFINITY
        } else {
            c / dy.abs()
        };

        // Cells are visited front to back, so the first hit is the closest one.
        loop {
            if self.is_lit(i, j) {
                if let Some(hit) = self.intersect_cell(ray, i, j, &bounds) {
                    return Some(hit);
                }
            }

            if t_next_x < t_next_y {
                let next_i = i as isize + step_i;
                if t_next_x > t_end || next_i < 0 || next_i >= self.width as isize {
                    return None;
                }
                i = next_i as usize;
                t_next_x += t_step_x;
            } else {
                let next_j = j as isize + step_j;
                if t_next_y > t_end || next_j < 0 || next_j >= self.height as isize {
                    return None;
                }
                j = next_j as usize;
                t_next_y += t_step_y;
            }
        }
    }

    fn bounding_box(&self) -> &AABB {
        return &self.bbox;
    }
}

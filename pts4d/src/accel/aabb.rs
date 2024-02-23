use cgmath::Vector3;

use crate::utils::vector_utils::{Interval, Ray};

trait HitableAccelStructure {
    fn intersect(&self, ray: &Ray, bounds: Interval) -> bool;
}

pub struct AABB {
    // Axis-Aligned Bounding Box.
    // An AABB is defined by three intervals
    // min-max X, Y and Z
    pub x_interval: Interval,
    pub y_interval: Interval,
    pub z_interval: Interval,
}

impl AABB {
    pub fn new_from_diagonals(d0: Vector3<f32>, d1: Vector3<f32>) -> AABB {
        // based on the two most far away points (connecting a diagonal)
        return AABB {
            x_interval: Interval::new(f32::min(d0.x, d1.x), f32::max(d0.x, d1.x)),
            y_interval: Interval::new(f32::min(d0.y, d1.y), f32::max(d0.y, d1.y)),
            z_interval: Interval::new(f32::min(d0.z, d1.z), f32::max(d0.z, d1.z)),
        };
    }

    pub fn new_from_intervals(x_i1: Interval, y_i2: Interval, z_i3: Interval) -> AABB {
        return AABB {
            x_interval: x_i1,
            y_interval: y_i2,
            z_interval: z_i3,
        };
    }

    pub fn new_from_aabbs(bb1: AABB, bb2: AABB) -> AABB {
        return AABB {
            x_interval: Interval::merge_intervals(bb1.x_interval, bb2.x_interval),
            y_interval: Interval::merge_intervals(bb1.y_interval, bb2.y_interval),
            z_interval: Interval::merge_intervals(bb1.z_interval, bb2.z_interval),
        };
    }

    pub fn get_axis(&self, i: usize) -> &Interval {
        match i {
            0 => &self.x_interval,
            1 => &self.y_interval,
            2 => &self.z_interval,
            _ => panic!("Index {} is not available on a 3D bounding-box", i),
        }
    }
}

impl HitableAccelStructure for AABB {
    fn intersect(&self, ray: &Ray, bounds: Interval) -> bool {
        let mut temp_bounds = bounds;

        for i in 0..2 {
            let inv_d = 1.0 / ray.direction[i];
            let orig = ray.origin[i];

            let mut t0 = (self.get_axis(i).min - orig) * inv_d;
            let mut t1 = (self.get_axis(i).max - orig) * inv_d;

            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            if t0 > temp_bounds.min {
                temp_bounds.min = t0;
            }

            if t1 < temp_bounds.max {
                temp_bounds.max = t1;
            }

            if temp_bounds.max <= temp_bounds.min {
                return false;
            }
        }

        return true;
    }
}

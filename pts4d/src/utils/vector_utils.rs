use cgmath::{InnerSpace, Vector3};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        return &self.origin + t * &self.direction;
    }
}

#[inline]
pub fn random_point_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    return loop {
        let temp = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vector3::new(1.0, 1.0, 1.0);

        if temp.magnitude2() < 1.0 {
            break temp;
        };
    };
}

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        return Interval { min, max };
    }

    pub fn merge_intervals(i1: Interval, i2: Interval) -> Interval {
        return Interval::new(f32::min(i1.min, i2.min), f32::max(i1.max, i2.max));
    }

    pub fn _size(&self) -> f32 {
        return self.max - self.min;
    }

    pub fn _expand(&self, delta: f32) -> Interval {
        let padding = delta / 2.0;
        return Interval {
            min: self.min - padding,
            max: self.max + padding,
        };
    }
}

#[inline]
pub fn is_close_to_zero(vector: Vector3<f32>) -> bool {
    let limit = 0.000001;
    if vector.x.abs() < limit && vector.y.abs() < limit && vector.y.abs() < limit {
        return true;
    }
    return false;
}

#[inline]
pub fn correct_face_normal(ray: &Ray, normal: Vector3<f32>) -> Vector3<f32> {
    if ray.direction.dot(normal) < 0.0 {
        return normal;
    } else {
        return -normal;
    }
}

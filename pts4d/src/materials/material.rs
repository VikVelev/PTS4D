use cgmath::Vector3;

use crate::utils::vector_utils::{Hit, Ray};

pub trait Reflective {
    type Material: Reflective;

    fn set_color(&mut self, color: Vector3<f32>) -> ();
    fn get_color(&self) -> Vector3<f32>;
    fn scatter(&self, ray: &Ray) -> Option<Hit<Self::Material>>;
}

#[derive(Copy, Clone)]
pub struct Metallic {
    pub color: Vector3<f32>,
}

impl Reflective for Metallic {
    type Material = Metallic;

    fn set_color(&mut self, color: Vector3<f32>) -> () {
        self.color = color;
    }

    fn get_color(&self) -> Vector3<f32> {
        return self.color;
    }

    fn scatter(&self, ray: &Ray) -> Option<Hit<Metallic>> {
        todo!()
    }
}

use crate::vector::Vector3;
use crate::color::Color;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction,
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<(Vector3, Color)>;
}

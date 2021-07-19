use crate::vector::{ Vector3 };
use crate::material::{ Material };

#[derive(Clone, Copy)]
pub struct Hit {
    pub position: Vector3,
    pub normal: Vector3,
    pub material: Material,
    pub distance: f64,
}

impl Hit {
    pub fn new(position: Vector3, normal: Vector3, material: Material, distance: f64) -> Self {
        Self {
            position,
            normal,
            material,
            distance,
        }
    }
}

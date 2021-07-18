use crate::vector::{ Vector3 };
use crate::color::{ Color };

#[derive(Clone, Copy)]
pub struct Hit {
    pub position: Vector3,
    pub normal: Vector3,
    pub color: Color,
}

impl Hit {
    pub fn new(position: Vector3, normal: Vector3, color: Color) -> Self {
        Self {
            position,
            normal,
            color,
        }
    }
}

use crate::color::{ Color };

#[derive(Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub emission: Color,
}

impl Material {
    pub fn new(diffuse: Color, emission: Color) -> Self {
        Self {
            diffuse,
            emission,
        }
    }
}

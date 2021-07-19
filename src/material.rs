use crate::color::{ Color };

#[derive(Clone, Copy)]
pub struct Material {
    pub diffuse_color: Color,
    pub emissive_color: Color,
    pub roughness: f64,
}

impl Material {
    pub fn new(diffuse_color: Color, emissive_color: Color, roughness: f64) -> Self {
        Self {
            diffuse_color,
            emissive_color,
            roughness,
        }
    }
}

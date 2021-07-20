use crate::vector::{ Vector3 };
use crate::hit::{ Hit };

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

use std::marker::{ Send, Sync };

pub trait Intersectable: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

mod vector;
mod ray;
mod sphere;
mod color;

use crate::vector::{ Vector3 };
use crate::ray::{ Ray, Intersectable };
use crate::sphere::{ Sphere };
use crate::color::Color;

fn main() {
    let sphere = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0);
    let ray = Ray::new(Vector3::new(2.0, 0.0, 0.0), Vector3::new(-1.0, 0.0, 0.0).normalized());

    match sphere.intersect(&ray) {
        Some((normal, color)) => {
            println!("{}, {}, {}", normal.x, normal.y, normal.z);
            println!("{}, {}, {}, {}", color.r, color.g, color.b, color.a);
        },
        _ => panic!(),
    }
}

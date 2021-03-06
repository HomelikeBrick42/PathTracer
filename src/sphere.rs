use crate::vector::{ Vector3 };
use crate::ray::{ Ray, Intersectable };
use crate::material::{ Material };
use crate::hit::{ Hit };

#[derive(Clone, Copy)]
pub struct Sphere {
    pub position: Vector3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64, material: Material) -> Self {
        Self {
            position,
            radius,
            material,
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let l = self.position - ray.origin;

        let tca = Vector3::dot(&l, &ray.direction);

        if tca < 0.0 {
            return Option::None;
        }

        let d2 = Vector3::dot(&l, &l) - tca * tca;
        if d2 > self.radius * self.radius {
            return Option::None;
        }

        let thc = (self.radius * self.radius - d2).sqrt();

        let mut t0 = tca - thc;
        let mut t1 = tca + thc;

        if t0 > t1 {
            let tmp = t0;
            t0 = t1;
            t1 = tmp;
        }

        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 {
                return Option::None;
            }
        }

        let t = t0;

        let hit_point = ray.origin + (ray.direction * t);
        let mut normal = (hit_point - self.position).normalized();
        if Vector3::dot(&normal, &ray.direction) > 0.0 {
            normal = normal * -1.0;
        }
        return Option::from(Hit::new(hit_point, normal, self.material, t));
    }
}

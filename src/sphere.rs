use std::rc::Rc;

use crate::vec3::Vec3;
use std::f32::consts::PI;
use rand::{self, Rng};

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3};
use crate::material::Material;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0. {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let hit_at = r.at(temp);
                let outward_normal = (hit_at - self.center) / self.radius;
                let is_front_face = r.direction.dot(outward_normal) < 0.;

                return Some(HitRecord::new(temp, hit_at, is_front_face, outward_normal, &self.material));
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let hit_at = r.at(temp);
                let outward_normal = (hit_at - self.center) / self.radius;
                let is_front_face = r.direction.dot(outward_normal) < 0.;

                return Some(HitRecord::new(temp, hit_at, is_front_face, outward_normal, &self.material));
            }
        }

        return None;
    }
}

// TODO implement traits in rand crate
pub fn random_in_unit_sphere() -> Point3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = Point3 {
            x: rng.gen_range(-1., 1.),
            y: rng.gen_range(-1., 1.),
            z: rng.gen_range(-1., 1.),
        };

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();

    let a: f32 = rng.gen_range(0., 2. * PI);
    let z: f32 = rng.gen_range(-1., 1.);
    let r: f32 = (1. - z * z).sqrt();

    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(*normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

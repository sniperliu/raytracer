use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(t: f32, spot: Point3, is_front_face: bool, outward_normal: Vec3) -> Self {
        HitRecord {
            t: t,
            p: spot,
            is_front_face: is_front_face,
            normal: if is_front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
}

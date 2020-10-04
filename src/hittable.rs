use std::rc::Rc;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;
use crate::aabb::AABB;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub is_front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

impl HitRecord {
    pub fn new(t: f32, spot: Point3, u: f32, v: f32, is_front_face: bool, outward_normal: Vec3, material: &Rc<dyn Material>) -> Self {
        HitRecord {
            t: t,
            p: spot,
            u: u,
            v: v,
            is_front_face: is_front_face,
            normal: if is_front_face {
                outward_normal
            } else {
                -outward_normal
            },
            material: Rc::clone(material),
        }
    }
}

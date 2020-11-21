use std::option::Option;
use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;
use crate::aabb::{AABB, surrounding_box};

pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Rc<dyn Material>,
}

impl Hittable for XYRect {

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(Point3::new(self.x0, self.y0, self.k - 0.0001),
                       Point3::new(self.x1, self.y1, self.k + 0.0001)))
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;

        if t < t_min || t > t_max {
            None
        } else {
            let x = r.origin.x + t * r.direction.x;
            let y = r.origin.y + t * r.direction.y;

            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                None
            } else {
                let u = (x - self.x0) / (self.x1 - self.x0);
                let v = (y - self.y0) / (self.y1 - self.y0);
                let outward_normal = Vec3::new(0., 0., 1.);
                let is_front_face = r.direction.dot(outward_normal) < 0.;

                Some(HitRecord::new(t, r.at(t), u, v, is_front_face, outward_normal, &self.material))
            }
        }
    }
}

pub struct XZRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Rc<dyn Material>,
}

impl Hittable for XZRect {

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(Point3::new(self.x0, self.k - 0.0001, self.z0,),
                       Point3::new(self.x1, self.k + 0.0001, self.z1,)))
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.direction.y;

        if t < t_min || t > t_max {
            None
        } else {
            let x = r.origin.x + t * r.direction.x;
            let z = r.origin.z + t * r.direction.z;

            if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
                None
            } else {
                let u = (x - self.x0) / (self.x1 - self.x0);
                let v = (z - self.z0) / (self.z1 - self.z0);
                let outward_normal = Vec3::new(0., 1., 0.);
                let is_front_face = r.direction.dot(outward_normal) < 0.;

                Some(HitRecord::new(t, r.at(t), u, v, is_front_face, outward_normal, &self.material))
            }
        }
    }
}

pub struct YZRect {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Rc<dyn Material>,
}

impl Hittable for YZRect {

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(Point3::new(self.k - 0.0001, self.y0, self.z0),
                       Point3::new(self.k + 0.0001, self.y1, self.z1)))
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / r.direction.x;

        if t < t_min || t > t_max {
            None
        } else {
            let y = r.origin.y + t * r.direction.y;
            let z = r.origin.z + t * r.direction.z;

            if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
                None
            } else {
                let u = (y - self.y0) / (self.y1 - self.y0);
                let v = (z - self.z0) / (self.z1 - self.z0);

                let outward_normal = Vec3::new(1., 0., 0.);
                let is_front_face = r.direction.dot(outward_normal) < 0.;

                Some(HitRecord::new(t, r.at(t), u, v, is_front_face, outward_normal, &self.material))
            }
        }
    }
}

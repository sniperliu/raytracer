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

pub struct Translate {
    pub object: Box<dyn Hittable>,
    pub offset: Vec3,
}

impl Hittable for Translate {

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if let Some(output_box) = self.object.bounding_box(time0, time1) {
            Some(AABB {
                min: output_box.min + self.offset,
                max: output_box.max + self.offset,
            })
        } else {
            None
        }
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray {
            origin: r.origin - self.offset,
            direction: r.direction,
            time: r.time
        };

        if let Some(mut hit_record) = self.object.hit(&moved_r, t_min, t_max) {
            hit_record.p += self.offset;
            let is_front_face = moved_r.direction.dot(hit_record.normal) < 0.;
            hit_record.is_front_face = is_front_face;
            hit_record.normal = if is_front_face { hit_record.normal } else { -hit_record.normal };

            Some(hit_record)
        } else {
            None
        }
    }
}

pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(p: Box<dyn Hittable>, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box(0., 1.);

        if let Some(b) = bbox {
            let mut min = Vec3{ x: f32::INFINITY, y: f32::INFINITY, z: f32::INFINITY };
            let mut max = Vec3{ x: f32::NEG_INFINITY, y: f32::NEG_INFINITY, z: f32::NEG_INFINITY };

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = (i as f32) * b.max.x + ((1 - i) as f32) * b.min.x;
                        let y = (j as f32) * b.max.y + ((1 - j) as f32) * b.min.y;
                        let z = (k as f32) * b.max.z + ((1 - k) as f32) * b.min.z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3 { x: new_x, y: y, z: new_z };

                        min.x = min.x.min(tester.x);
                        min.y = min.y.min(tester.y);
                        min.z = min.z.min(tester.z);

                        max.x = max.x.max(tester.x);
                        max.y = max.y.max(tester.y);
                        max.z = max.z.max(tester.z);
                    }
                }
            }

            Self {
                object: p,
                sin_theta: sin_theta,
                cos_theta: cos_theta,
                bbox: Some(AABB { min: min, max: max }),
            }
        } else {
            Self {
                object: p,
                sin_theta: sin_theta,
                cos_theta: cos_theta,
                bbox: None
            }
        }
    }
}

impl Hittable for RotateY {

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        if let Some(bbox) = &self.bbox {
            Some(AABB { min: bbox.min, max: bbox.max })
        } else {
            None
        }
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.x = self.cos_theta * r.origin[0] - self.sin_theta * r.origin[2];
        origin.z = self.sin_theta * r.origin[0] + self.cos_theta * r.origin[2];

        direction.x = self.cos_theta * r.direction[0] - self.sin_theta * r.direction[2];
        direction.z = self.sin_theta * r.direction[0] + self.cos_theta * r.direction[2];

        let rotated_r = Ray { origin: origin, direction: direction, time: r.time };

        if let Some(mut record) = self.object.hit(&rotated_r, t_min, t_max) {
            let mut p = record.p;
            let mut normal = record.normal;

            p.x = self.cos_theta * record.p[0] + self.sin_theta * record.p[2];
            p.z = - self.sin_theta  * record.p[0] + self.cos_theta * record.p[2];

            normal.x = self.cos_theta * record.normal[0] + self.sin_theta * record.normal[2];
            normal.z = - self.sin_theta * record.normal[0] + self.cos_theta * record.normal[2];

            record.p = p;
            let is_front_face = rotated_r.direction.dot(record.normal) < 0.;
            record.is_front_face = is_front_face;
            record.normal = if is_front_face { normal } else { -normal };

            Some(record)
        } else {
            None
        }
    }
}

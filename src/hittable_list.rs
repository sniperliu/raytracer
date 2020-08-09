use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::vec3::Point3;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        return temp_rec;
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            None
        } else {

            let mut output_box = None;

            for object in &self.objects {
                if let Some(temp_box) = object.bounding_box(t0, t1) {
                    output_box = if let Some(obox) = output_box {
                        Some(surrounding_box(&obox, &temp_box))
                    } else {
                        Some(temp_box)
                    }
                } else {
                    return None;
                }
            }

            output_box
        }
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point3::new(box0.min.x.min(box1.min.x),
                            box0.min.y.min(box1.min.y),
                            box0.min.z.min(box1.min.z));
    let big = Point3::new(box0.max.x.max(box1.max.x),
                          box0.max.y.max(box1.max.y),
                          box0.max.z.max(box1.max.z));

    AABB::new(small, big)
}

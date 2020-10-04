use crate::vec3::Point3;
use crate::ray::Ray;

// axis-aligned bounding boxes
#[derive(Clone)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        AABB {
            min: a,
            max: b,
        }
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            // TODO check float NaN & Infinit later
            let inv_d = 1. / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) / inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) / inv_d;
            if inv_d < 0. {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tmin = if t0 > tmin { t0 } else { tmin };
            let tmax = if t1 > tmax { t1 } else { tmax };

            if tmax <= tmin {
                return false;
            }
        }

        return true;
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

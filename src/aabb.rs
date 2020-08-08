use crate::vec3::Point3;
use crate::ray::Ray;

// axis-aligned bounding boxes
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub new(a: Point3, b: Point3) -> Self {
        AABB {
            min: a,
            max: b,
        }
    }

    pub hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
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

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
            let t0 = ((self.min[a] - r.origin[a]) / r.direction[a]).min((self.max[a] - r.origin[a]) / r.direction[a]);
            let t1 = ((self.min[a] - r.origin[a]) / r.direction[a]).max((self.max[a] - r.origin[a]) / r.direction[a]);
            let tmin = t0.max(tmin);
            let tmax = t1.min(tmax);
            if tmax <= tmin {
                return false;
            }
        }

        return true;
    }
}

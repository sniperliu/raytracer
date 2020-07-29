use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::vec3::{Point3, Vec3};

struct Sphere {
    center: Point3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0. {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

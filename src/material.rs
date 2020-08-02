use crate::sphere::random_in_unit_sphere;
use crate::vec3::Vec3;
use crate::sphere::random_unit_vector;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;

pub trait Material {
    fn scatter(&self, spot: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = rec.normal + random_unit_vector();
        let scattered = Ray{
            origin: rec.p,
            direction: scatter_direction,
        };
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - 2. * (*v).dot(*n) * (*n)
}

pub struct Metal {
    pub albedo: Color,
    pub fuzzy: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzzy: f32) -> Self {
        Metal {
            albedo: albedo,
            fuzzy: fuzzy.min(1.0),
        }
    }
}

impl Material for Metal {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&r_in.direction.normalize(), &rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzzy * random_in_unit_sphere(),
        };
        if scattered.direction.dot(rec.normal) > 0. {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

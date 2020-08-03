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

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv).dot(n);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt() * n);
    r_out_perp + r_out_parallel
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> std::option::Option<(Ray, Color)> {
        let attenuation = Color(Vec3::new(1., 1., 1.));
        let etai_over_etat = if rec.is_front_face { 1.0 / self.ref_idx } else { self.ref_idx };

        let unit_direction = r_in.direction.normalize();
        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        let scattered = Ray {
            origin: rec.p,
            direction: refracted,
        };

        Some((scattered, attenuation))
    }
}

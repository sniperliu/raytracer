use crate::texture::{Texture, SolidColor};
use crate::sphere::random_in_unit_sphere;
use crate::vec3::{Point3, Vec3};
use crate::sphere::random_unit_vector;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;

pub trait Material {
    fn emitted(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        Color(Vec3::new(0., 0., 0.))
    }

    fn scatter(&self, spot: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new_from_texture(texture: Box<dyn Texture>) -> Self {
        Lambertian{ albedo: texture }
    }

    pub fn new_from_color(color: Color) -> Self{
        Lambertian{ albedo: Box::new(SolidColor::new_from_color(color)) }
    }
}

impl Material for Lambertian {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = rec.normal + random_unit_vector();
        let scattered = Ray::new(rec.p, scatter_direction, r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
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
        let scattered = Ray::new_without_move(rec.p, reflected + self.fuzzy * random_in_unit_sphere());
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

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;

    r0 + (1. - r0) * (1. - cosine).powi(5)
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> std::option::Option<(Ray, Color)> {
        let attenuation = Color(Vec3::new(1., 1., 1.));
        let etai_over_etat = if rec.is_front_face { 1.0 / self.ref_idx } else { self.ref_idx };

        let unit_direction = r_in.direction.normalize();

        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &rec.normal);
            let scattered = Ray::new_without_move(rec.p, reflected);
            Some((scattered, attenuation))
        } else if rand::random::<f32>() < schlick(cos_theta, etai_over_etat) {
            let reflected = reflect(&unit_direction, &rec.normal);
            let scattered = Ray::new_without_move(rec.p, reflected);
            Some((scattered, attenuation))
        } else {
            let refracted = refract(unit_direction, rec.normal, etai_over_etat);
            let scattered = Ray::new_without_move(rec.p, refracted);
            Some((scattered, attenuation))
        }

    }
}

pub struct DiffuseLight {
    pub emit: Box<dyn Texture>,
}

impl Material for DiffuseLight {

    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }

    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

}

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    // vertical field of view in degrees
    pub fn new(vfov: f32, aspect_ratio: f32) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();

        // let aspect_ratio: f32 = 16.0 / 9.0;
        let viewport_height: f32 = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length: f32 = 1.0;

        let origin = Vec3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}

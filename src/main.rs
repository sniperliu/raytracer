use std::io::{self, Write};

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;

use hittable::Hittable;
use crate::hittable_list::HittableList;
use sphere::Sphere;
use vec3::Vec3;

fn ray_color(r: &ray::Ray, w: &HittableList) -> color::Color {
    if let Some(rec) = w.hit(&r, 0., f32::MAX) {
        color::Color(0.5 * (rec.normal + Vec3::new(1., 1., 1.)))
    } else {
        let direction = r.direction.normalize();
        let t = 0.5 * (direction.y + 1.);
        color::Color(vec3::Vec3::new(1., 1., 1.) * (1.0 - t) +
                     t * vec3::Vec3::new(0.5, 0.7, 1.))
    }
}

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;

    let mut world = HittableList{ objects: Vec::new(), };
    world.add(Box::new(Sphere{ center: Vec3::new(0., 0., -1.), radius: 0.5 }));
    world.add(Box::new(Sphere{ center: Vec3::new(0., -100.5, -1.), radius: 100. }));

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = vec3::Vec3::new(0., 0., 0.);
    let horizontal = vec3::Vec3::new(viewport_width, 0., 0.);
    let vertical = vec3::Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3::new(0., 0., focal_length);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let stderr = io::stderr();
    let mut err_handle = stderr.lock();

    write!(handle, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for j in 0..image_height {
        write!(err_handle, "\rScanlines remaining: {} ", j).unwrap();
        err_handle.flush().unwrap();
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = ray::Ray{
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            // let color = color::Color(vec3::Vec3{ x: i as f32 / (image_width - 1) as f32,
            //                                      y: j as f32 / (image_height - 1) as f32,
            //                                      z: 0.25 });
            let color = ray_color(&r, &world);

            write!(handle, "{}", color).unwrap();
        }
    }

    write!(err_handle, "\nDone.\n").unwrap();
}

use crate::color::Color;
use std::io::{self, Write};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::hittable_list::HittableList;
use camera::Camera;
use hittable::Hittable;
use sphere::Sphere;
use vec3::Vec3;

use rand::Rng;

fn ray_color(r: &ray::Ray, w: &HittableList) -> color::Color {
    if let Some(rec) = w.hit(&r, 0., f32::MAX) {
        color::Color(0.5 * (rec.normal + Vec3::new(1., 1., 1.)))
    } else {
        let direction = r.direction.normalize();
        let t = 0.5 * (direction.y + 1.);
        color::Color(vec3::Vec3::new(1., 1., 1.) * (1.0 - t) + t * vec3::Vec3::new(0.5, 0.7, 1.))
    }
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

fn write_color(out: &std::io::Stdout, pixel_color: Color, samples_per_pixel: i32) {
    let mut handle = out.lock();

    let mut r = pixel_color.0.x;
    let mut g = pixel_color.0.y;
    let mut b = pixel_color.0.z;

    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    write!(
        handle,
        "{} {} {}\n",
        (256. * clamp(r, 0.0, 0.999)) as u8,
        (256. * clamp(g, 0.0, 0.999)) as u8,
        (256. * clamp(b, 0.0, 0.999)) as u8,
    )
    .unwrap();
}

fn main() {
    let mut rng = rand::thread_rng();

    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel = 100;

    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.add(Box::new(Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
    }));

    // Camera
    let cam = Camera::new();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let stderr = io::stderr();
    let mut err_handle = stderr.lock();

    write!(handle, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for j in (0..image_height).rev() {
        write!(err_handle, "\rScanlines remaining: {} ", j).unwrap();
        err_handle.flush().unwrap();
        for i in 0..image_width {
            let mut v_pixel_color = Vec3::new(0., 0., 0.);
            for _s in 0..samples_per_pixel {
                let u = ((i as f32) + rng.gen::<f32>()) / ((image_width - 1) as f32);
                let v = ((j as f32) + rng.gen::<f32>()) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                v_pixel_color += ray_color(&r, &world).0;
            }

            write_color(&stdout, Color(v_pixel_color), samples_per_pixel);
        }
    }

    write!(err_handle, "\nDone.\n").unwrap();
}

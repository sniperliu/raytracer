use crate::material::Dielectric;
use crate::color::Color;
use std::io::{self, Write};
use std::rc::Rc;
use std::f32::consts::PI;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod material;

use crate::hittable_list::HittableList;
use camera::Camera;
use hittable::Hittable;
use sphere::{Sphere, random_in_hemisphere};
use vec3::Vec3;
use ray::Ray;
use material::{Lambertian, Metal};

use rand::Rng;

fn ray_color(r: &ray::Ray, w: &HittableList, depth: i32) -> color::Color {
    if depth <= 0 {
        return color::Color(Vec3::new(0., 0., 0.));
    }

    if let Some(rec) = w.hit(&r, 0.001, f32::MAX) {
        if let Some((scattered, attenuation)) = rec.material.scatter(&r, &rec) {
            Color(attenuation.0 * ray_color(&scattered, w, depth - 1).0)
        } else {
            Color(Vec3::new(0., 0., 0.))
        }
        // let target = rec.p + rec.normal + random_in_hemisphere(&rec.normal);
        // let ray = Ray{ origin: rec.p, direction: target - rec.p};
        // color::Color(0.5 * ray_color(&ray, w, depth - 1).0)
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
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

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
    let max_depth = 50;

    let R = (PI / 4.).cos();
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let material_ground = Rc::new(Lambertian{ albedo: Color(Vec3::new(0.8, 0.8, 0.)) });
    let material_center = Rc::new(Lambertian{ albedo: Color(Vec3::new(0.1, 0.2, 0.5)) });
    let material_left = Rc::new(Dielectric{ ref_idx: 1.5 });
    let material_right = Rc::new(Metal::new(Color(Vec3::new(0.8, 0.6, 0.2)), 0.));

    world.add(Box::new(Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
        material: material_ground,
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
        material: material_center,
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(-1., 0., -1.),
        radius: 0.5,
        material: material_left.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(-1., 0., -1.),
        radius: -0.45,
        material: material_left.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(1., 0., -1.),
        radius: 0.5,
        material: material_right,
    }));

    // Camera
    let cam = Camera::new(Vec3::new(-2., 2., 1.), Vec3::new(0., 0., -1.), Vec3::new(0., 1., 0.), 20., aspect_ratio);

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
                v_pixel_color += ray_color(&r, &world, max_depth).0;
            }

            write_color(&stdout, Color(v_pixel_color), samples_per_pixel);
        }
    }

    write!(err_handle, "\nDone.\n").unwrap();
}

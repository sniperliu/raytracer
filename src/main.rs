use crate::perlin::Perlin;
use crate::material::Dielectric;
use crate::color::Color;
use std::io::{self, Write};
use std::rc::Rc;

mod aabb;
// mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod material;
mod texture;
mod perlin;

use crate::hittable_list::HittableList;
use camera::Camera;
use hittable::Hittable;
use sphere::{Sphere, MovingSphere, random_in_hemisphere};
use vec3::Vec3;
use ray::Ray;
use texture::{CheckerTexture, NoiseTexture};
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

fn two_spheres() -> HittableList {
    let mut spheres = HittableList{
        objects: Vec::new(),
    };

    let checker_texture = CheckerTexture::new(Color(Vec3::new(0.2, 0.3, 0.1)), Color(Vec3::new(0.9, 0.9, 0.9)));
    let checker_material = Rc::new(Lambertian::new_from_texture(Box::new(checker_texture)));

    spheres.add(Box::new(Sphere {
        center: Vec3::new(0., -10., 0.),
        radius: 10.,
        material: checker_material.clone(),
    }));
    spheres.add(Box::new(Sphere {
        center: Vec3::new(0., 10., 0.),
        radius: 10.,
        material: checker_material.clone(),
    }));

    spheres
}

fn two_perlin_spheres() -> HittableList {
    let mut spheres = HittableList{
        objects: Vec::new(),
    };

    let texture = NoiseTexture{ noise: Perlin::new(), scale: 4., };
    let material = Rc::new(Lambertian::new_from_texture(Box::new(texture)));

    spheres.add(Box::new(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: material.clone(),
    }));
    spheres.add(Box::new(Sphere {
        center: Vec3::new(0., 2., 0.),
        radius: 2.,
        material: material.clone(),
    }));

    spheres
}

fn random_scene() -> HittableList {
    let mut world = HittableList {
        objects: Vec::new(),
    };

    // let material_ground = Rc::new(Lambertian::new_from_color(Color(Vec3::new(0.5, 0.5, 0.5))));
    let material_checker = Rc::new(Lambertian::new_from_texture(
        Box::new(CheckerTexture::new(Color(Vec3::new(0.2, 0.3, 0.1)), Color(Vec3::new(0.9, 0.9, 0.9))))));
    world.add(Box::new(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: material_checker,
    }));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(),
                                   0.2,
                                   b as f32 + 0.8 * rng.gen::<f32>());

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color(rng.gen::<Vec3>() * rng.gen::<Vec3>());
                    let center2 = center + Vec3::new(0., rng.gen_range(0., 0.5), 0.);
                    world.add(Box::new(MovingSphere {
                        center0: center,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Rc::new(Lambertian::new_from_color(albedo)),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color(rng.gen::<Vec3>());
                    let fuzz = rng.gen();
                    world.add(Box::new(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Rc::new(Metal::new(albedo, fuzz)),
                    }));
                } else {
                    // glass
                    world.add(Box::new(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Rc::new(Dielectric{ ref_idx: 1.5 }),
                    }));
                }

            }
        }
    }

    let material1 = Rc::new(Dielectric{ ref_idx: 1.5 });
    world.add(Box::new(Sphere {
        center: Vec3::new(0., 1., 0.),
        radius: 1.,
        material: material1,
    }));

    let material2 = Rc::new(Lambertian::new_from_color(Color(Vec3::new(0.4, 0.2, 0.1))));
    world.add(Box::new(Sphere {
        center: Vec3::new(-4., 1., 0.),
        radius: 1.,
        material: material2,
    }));

    let material3 = Rc::new(Metal::new(Color(Vec3::new(0.7, 0.6, 0.5)), 0.));
    world.add(Box::new(Sphere {
        center: Vec3::new(4., 1., 0.),
        radius: 1.,
        material: material3,
    }));

    world
}

fn main() {
    let mut rng = rand::thread_rng();

    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let world: HittableList;
    let look_from;
    let look_at;
    let vfov;
    let mut aperture = 0.0;
    match 0 {
        1 => {
            world = random_scene();
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.0;
            aperture = 0.1;
        },
        2 => {
            world = two_spheres();
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.0;
        },
        _ => {
            world = two_perlin_spheres();
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.0;
        },
    }

    // Camera
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.0;
    let cam = Camera::new(look_from, look_at, vup, vfov, aspect_ratio, aperture, dist_to_focus, 0., 1.);

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

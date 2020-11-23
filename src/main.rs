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
mod aarect;
mod r#box;

use crate::hittable_list::HittableList;
use camera::Camera;
use hittable::{Hittable, Translate, RotateY};
use sphere::{Sphere, MovingSphere, random_in_hemisphere};
use vec3::Vec3;
use ray::Ray;
use texture::{CheckerTexture, NoiseTexture, ImageTexture};
use material::{Lambertian, Metal, DiffuseLight};
use rand::Rng;
use aarect::{XYRect, YZRect, XZRect};

fn ray_color(r: &ray::Ray, background: Color,  w: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return color::Color(Vec3::new(0., 0., 0.));
    }


    if let Some(rec) = w.hit(&r, 0.001, f32::MAX) {
        let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);

        if let Some((scattered, attenuation)) = rec.material.scatter(&r, &rec) {
            Color(emitted.0 + attenuation.0 * ray_color(&scattered, background, w, depth - 1).0)
        } else {
            emitted
        }
    } else {
        background
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

fn earth() -> HittableList {
    let earth_texture = ImageTexture::new("./img/earthmap.jpg");
    let earth_surface = Rc::new(Lambertian::new_from_texture(Box::new(earth_texture)));
    let globe = Sphere {
        center: Vec3::new(0., 0., 0.),
        radius: 2.,
        material: earth_surface.clone(),
    };

    HittableList {
        objects: vec![Box::new(globe)],
    }
}

fn simple_light() -> HittableList {
    let mut objects = HittableList {
        objects: Vec::new(),
    };

    let texture = NoiseTexture{ noise: Perlin::new(), scale: 4., };
    let material = Rc::new(Lambertian::new_from_texture(Box::new(texture)));

    objects.add(Box::new(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: material.clone(),
    }));
    objects.add(Box::new(Sphere {
        center: Vec3::new(0., 2., 0.),
        radius: 2.,
        material: material.clone(),
    }));
    objects.add(Box::new(XYRect {
        x0: 3., x1: 5., y0: 1., y1: 3., k: -2.,
        material: Rc::new(DiffuseLight::new(Color(Vec3::new(4., 4., 4.)))),
    }));


    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList {
        objects: Vec::new(),
    };

    let red = Rc::new(Lambertian::new_from_color(Color(Vec3::new(0.65, 0.05, 0.05))));
    let white = Rc::new(Lambertian::new_from_color(Color(Vec3::new(0.73, 0.73, 0.73))));
    let green = Rc::new(Lambertian::new_from_color(Color(Vec3::new(0.12, 0.45, 0.15))));
    let light = Rc::new(DiffuseLight::new(Color(Vec3::new(15., 15., 15.))));

    objects.add(Box::new(YZRect {
        y0: 0., y1: 555., z0: 0., z1: 555., k: 555., material: green.clone(),
    }));
    objects.add(Box::new(YZRect {
        y0: 0., y1: 555., z0: 0., z1: 555., k: 0., material: red.clone(),
    }));
    objects.add(Box::new(XZRect {
        x0: 213., x1: 343., z0: 227., z1: 332., k: 554., material: light.clone(),
    }));
    objects.add(Box::new(XZRect {
        x0: 0., x1: 555., z0: 0., z1: 555., k: 0., material: white.clone(),
    }));
    objects.add(Box::new(XZRect {
        x0: 0., x1: 555., z0: 0., z1: 555., k: 555., material: white.clone(),
    }));
    objects.add(Box::new(XYRect {
        x0: 0., x1: 555., y0: 0., y1: 555., k: 555., material: white.clone(),
    }));

    let box1 = Translate {
        object: Box::new(RotateY::new(Box::new(r#box::Box::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), white.clone())), 15.)),
        offset: Vec3::new(265., 0., 295.)};
    objects.add(Box::new(box1));
    let box2 = Translate {
        object: Box::new(RotateY::new(Box::new(r#box::Box::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white.clone())), -18.)),
        offset: Vec3::new(130., 0., 65.),};
    objects.add(Box::new(box2));

    objects
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
    let mut aspect_ratio: f32 = 16.0 / 9.0;
    let mut image_width: usize = 400;
    let mut image_height: usize = (image_width as f32 / aspect_ratio) as usize;
    let mut samples_per_pixel = 100;
    let max_depth = 50;

    let world: HittableList;
    let look_from;
    let look_at;
    let vfov;
    let mut aperture = 0.0;
    let mut background = Color(Vec3::new(0., 0., 0.));

    match 0 {
        1 => {
            world = random_scene();
            background = Color(Vec3::new(0.7, 0.8, 1.));
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.0;
            aperture = 0.1;
        },
        2 => {
            world = two_spheres();
            background = Color(Vec3::new(0.7, 0.8, 1.));
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.0;
        },
        3 => {
            world = two_perlin_spheres();
            background = Color(Vec3::new(0.7, 0.8, 1.));
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.0;
        },
        4 => {
            world = earth();
            background = Color(Vec3::new(0.7, 0.8, 1.));
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.;
        },
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color(Vec3::new(0., 0., 0.));
            look_from = Vec3::new(26., 3., 6.);
            look_at = Vec3::new(0., 2., 0.);
            vfov = 20.;
        },
        _ => {
            world = cornell_box();
            aspect_ratio = 1.;
            image_width = 600;
            image_height = (image_width as f32 / aspect_ratio) as usize;
            samples_per_pixel = 200;
            background = Color(Vec3::new(0., 0., 0.));
            look_from = Vec3::new(278., 278., -800.);
            look_at = Vec3::new(278., 278., 0.);
            vfov = 40.;
        }
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
                v_pixel_color += ray_color(&r, background, &world, max_depth).0;
            }

            write_color(&stdout, Color(v_pixel_color), samples_per_pixel);
        }
    }

    write!(err_handle, "\nDone.\n").unwrap();
}

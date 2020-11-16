use crate::Vec3;
use crate::vec3::{self, Point3};
use crate::color::{Color};
use crate::perlin::{Perlin};

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

pub struct SolidColor {
    color: Color
}

impl SolidColor {
    pub fn new_from_color(c: Color) -> Self {
        SolidColor { color: c }
    }

    pub fn new_from_raw(r: f32, g: f32, b: f32) -> Self {
        SolidColor { color: Color(vec3::Vec3{ x: r, y: g, z: b }) }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> Self {
        Self {
            odd: Box::new(SolidColor::new_from_color(c1)),
            even: Box::new(SolidColor::new_from_color(c2)),
        }
    }
}

impl Texture for CheckerTexture {

    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        // What is sines?
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }

}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f32,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Point3) -> Color {
        Color(Vec3::new(1., 1., 1.) * 0.5 * (1. + (self.scale * p.z + 10. * self.noise.turb(&(self.scale * *p), 7)).sin()))
    }
}

use image::{open, DynamicImage, GenericImageView, Pixel, Rgb};

pub struct ImageTexture {
    pub data: DynamicImage,
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

impl ImageTexture {
    pub fn new(fname: &str) -> Self {
        ImageTexture {
            data: open(fname).unwrap(),
        }
    }
}

impl Texture for ImageTexture {

    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let u = clamp(u, 0., 1.);
        let v = 1.0 - clamp(v, 0., 1.);

        let (width, height) = self.data.dimensions();
        let mut i = (u * width as f32) as u32;
        let mut j = (v * height as f32) as u32;

        if i >= width {
            i = width - 1;
        }
        if j >= height {
            j = height - 1;
        }

        let color_scale = 1. / 255.;
        let Rgb([r, g, b]) = self.data.get_pixel(i, j).to_rgb();

        Color(Vec3{ x: r as f32 * color_scale, y: g as f32 * color_scale, z: b as f32 * color_scale})
    }
}

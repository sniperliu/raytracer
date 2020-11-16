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
        Color(Vec3::new(1., 1., 1.) * self.noise.turb(&(self.scale * *p), 7))
    }
}

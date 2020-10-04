use crate::vec3::{self, Point3};
use crate::color::{Color};

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> &Color;
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
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> &Color {
        &self.color
    }
}

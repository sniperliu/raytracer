use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Ray {
            origin: origin,
            direction: direction,
            time: time,
        }
    }

    pub fn new_without_move(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction,
            time: 0.,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

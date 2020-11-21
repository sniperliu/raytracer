use crate::Ray;
use crate::aabb::AABB;
use crate::hittable::HitRecord;
use std::rc::Rc;
use crate::material::Material;
use crate::aarect::*;
use crate::hittable_list::HittableList;
use crate::hittable::Hittable;
use crate::vec3::*;

pub struct Box {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl Box {
    pub fn new(p0: Point3, p1: Point3, material: Rc<dyn Material>) -> Self {
        let mut sides = HittableList { objects: Vec::new() };

        sides.add(std::boxed::Box::new(XYRect{ x0: p0.x, x1: p1.x, y0: p0.y, y1: p1.y, k: p1.z, material: material.clone()}));
        sides.add(std::boxed::Box::new(XYRect{ x0: p0.x, x1: p1.x, y0: p0.y, y1: p1.y, k: p0.z, material: material.clone()}));

        sides.add(std::boxed::Box::new(XZRect{ x0: p0.x, x1: p1.x, z0: p0.z, z1: p1.z, k: p1.y, material: material.clone()}));
        sides.add(std::boxed::Box::new(XZRect{ x0: p0.x, x1: p1.x, z0: p0.z, z1: p1.z, k: p0.y, material: material.clone()}));

        sides.add(std::boxed::Box::new(YZRect{ y0: p0.y, y1: p1.y, z0: p0.z, z1: p1.z, k: p1.x, material: material.clone()}));
        sides.add(std::boxed::Box::new(YZRect{ y0: p0.y, y1: p1.y, z0: p0.z, z1: p1.z, k: p1.x, material: material.clone()}));

        Self {
            box_min: p0,
            box_max: p1,
            sides: sides,
        }
    }
}

impl Hittable for Box {

    fn bounding_box(&self, _t_min: f32, _t_max: f32) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

}

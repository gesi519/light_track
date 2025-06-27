use crate::vec3::{Vec3, Point3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub orig : Point3,
    pub dir : Vec3,
}

impl Ray {
    
    pub fn new(origi: Point3, direct: Vec3) -> Self {
        Self { orig : origi, dir : direct }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t : f64) -> Point3 {
        self.orig + t * self.dir
    }

}

pub fn hit_sphere(center : Point3, radius : f64, r : Ray) -> f64 {
    let oc = center - *r.origin();
    let a = Vec3::dot(*r.direction(), *r.direction());
    //  let b = -2.0 * Vec3::dot(*r.direction(), oc);
    let h = Vec3::dot(*r.direction(), oc);
    let c = Vec3::dot(oc, oc) - radius * radius;
    //  let discriminant = b * b - 4.0 * a * c;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    }else {
        //  (-b - discriminant.sqrt()) / (2.0 * a)
        (h - discriminant.sqrt()) / a
    }
}
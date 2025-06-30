use crate::hittable::{Hittable,HitRecord};
use crate::ray::Ray;
use crate::Arc;
use crate::material::Material;
use crate::vec3::{Vec3,Point3};
use crate::interval::Interval;

pub struct Sphere {
    pub center : Ray,
    pub radius : f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new_stationary(center1 : Point3, radius1 : f64, mat : Arc<dyn Material>) -> Self {
        Self {  center : Ray::from(center1, Vec3::new(0.0, 0.0, 0.0)),
                radius : radius1.max(0.0),
                mat    : mat,
             }
    }

    // 创建运动球体
    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::from(center1, center2 - center1), // 线性运动
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r : &Ray, ray_t : &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(r.direction(),&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root  = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);

        let outward_normal = (p - current_center) / self.radius;
        
        let mat = Arc::clone(&self.mat);
        let mut rec = HitRecord { p, normal : Vec3::new(0.0, 0.0, 0.0), t : root, front_face : true, mat : mat };
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
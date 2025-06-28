use crate::material::Material;
use crate::vec3::{Point3,Vec3};
use crate::ray::Ray;
use std::sync::Arc;
use crate::interval::Interval;

pub trait Hittable {
    fn hit(&self, r : &Ray, ray_t : &Interval) -> Option<HitRecord>;
}

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p : Point3,         // 交点坐标
    pub normal : Vec3,      // 法向量（始终指向射线反方向）
    pub t : f64,            // 命中的射线参数 t
    pub front_face : bool,  // 射线是否从外部命中物体
    pub mat: Arc<dyn Material>, 
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r : &Ray, outward_normal : &Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        }else {
            -*outward_normal
        };
    }
}

pub struct Sphere {
    pub center : Point3,
    pub radius : f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center1 : Point3, radius1 : f64, mat : Arc<dyn Material>) -> Self {
        Self {  center : center1,
                radius : radius1.max(0.0),
                mat    : mat,
             }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r : &Ray, ray_t : &Interval) -> Option<HitRecord> {
        let oc = self.center - *r.origin();
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
        let normal = (p - self.center) / self.radius;
        let outward_normal = (p - self.center) / self.radius;
        
        let mat = Arc::clone(&self.mat);
        let mut rec = HitRecord { p, normal, t : root, front_face : true, mat : mat };
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn with_object(object: Arc<dyn Hittable + Send + Sync>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t : &Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record_result: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(ray, &Interval::new(ray_t.min,closest_so_far)) {
                closest_so_far = temp_rec.t;
                hit_record_result = Some(temp_rec);
            }
        }

        hit_record_result
    }
}
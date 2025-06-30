use crate::material::Material;
use crate::vec3::{Point3,Vec3};
use crate::ray::Ray;
use std::sync::Arc;
use crate::interval::Interval;
use crate::AABB::Aabb;

pub trait Hittable : Send + Sync {
    fn hit(&self, r : &Ray, ray_t : &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> Aabb;
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

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub bbox : Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::new_empty(), // 或 Aabb::default()
        }
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
        self.bbox = Aabb::surrounding_box(&self.bbox, &object.bounding_box());
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

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
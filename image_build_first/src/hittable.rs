use crate::AABB::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub trait Hittable: Send + Sync {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'a>>;
    fn bounding_box(&self) -> Aabb;

    fn pdf_value(&self, _origin : &Point3, _direction: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, _origin : &Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,        // 交点坐标
    pub normal: Vec3,     // 法向量（始终指向射线反方向）
    pub t: f64,           // 命中的射线参数 t
    pub front_face: bool, // 射线是否从外部命中物体
    pub mat: &'a dyn Material,
    pub u: f64,
    pub v: f64,
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::empty(), // 或 Aabb::default()
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
    fn hit<'a>(&'a self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord<'a>> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record_result: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(ray, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp_rec.t;
                hit_record_result = Some(temp_rec);
            }
        }

        hit_record_result
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        if self.objects.is_empty() {
            return 0.0;
        }

        let weight = 1.0 / self.objects.len() as f64;
        self.objects.iter().map(|object| weight * object.pdf_value(origin, direction)).sum()
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let len = self.objects.len();
        if len == 0 {
            return Vec3::new(1.0, 0.0, 0.0); // fallback direction
        }

        let idx = rtweekend::random_int(0, len - 1);
        self.objects[idx].random(origin)
    }
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object: object.clone(),
            offset: offset,
            bbox: bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'a>> {
        let offset_r = Ray::new(*r.origin() - self.offset, *r.direction(), r.time());

        if let Some(rec) = self.object.hit(&offset_r, ray_t) {
            Some(HitRecord {
                t: rec.t,
                p: rec.p + self.offset,
                normal: rec.normal,
                front_face: rec.front_face,
                mat: rec.mat,
                u: rec.u,
                v: rec.v,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = rtweekend::degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box();
        let mut min = Point3::new(
            rtweekend::INFINITY_F64,
            rtweekend::INFINITY_F64,
            rtweekend::INFINITY_F64,
        );
        let mut max = Point3::new(
            -rtweekend::INFINITY_F64,
            -rtweekend::INFINITY_F64,
            -rtweekend::INFINITY_F64,
        );

        for i in 0..2 {
            let bi = i as f64;
            for j in 0..2 {
                let bj = j as f64;
                for k in 0..2 {
                    let bk = k as f64;
                    let x = bi * bbox.x.max + (1.0 - bi) * bbox.x.min;
                    let y = bj * bbox.y.max + (1.0 - bj) * bbox.y.min;
                    let z = bk * bbox.z.max + (1.0 - bk) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            object: object,
            sin_theta: sin_theta,
            cos_theta: cos_theta,
            bbox: Aabb::from_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'a>> {
        let origin = Point3::new(
            self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z(),
            r.origin().y(),
            self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z(),
        );
        let direction = Point3::new(
            self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z(),
            r.direction().y(),
            self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z(),
        );

        let rotate_r = Ray::new(origin, direction, r.time());

        if let Some(rec) = self.object.hit(&rotate_r, ray_t) {
            let p = Point3::new(
                self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z(),
                rec.p.y(),
                -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z(),
            );
            let normal = Point3::new(
                self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z(),
                rec.normal.y(),
                -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z(),
            );
            Some(HitRecord {
                t: rec.t,
                p,
                normal,
                front_face: rec.front_face,
                mat: rec.mat,
                u: rec.u,
                v: rec.v,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

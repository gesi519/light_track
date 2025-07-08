use crate::AABB::Aabb;
use crate::vec3::{Point3, Vec3};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::rtweekend;
use crate::material::Material;
use std::sync::Arc;

pub struct Triangle {
    pub v0: Point3,             // 顶点 0
    pub v1: Point3,             // 顶点 1
    pub v2: Point3,             // 顶点 2
    pub normal: Vec3,           // 三角形的单位法向量（由 v0v1 与 v0v2 的叉积归一化而来）
    pub mat: Arc<dyn Material>, // 材质指针
    pub bbox: Aabb,             // 用于 BVH 的包围盒
    pub area: f64,              // 三角形面积，用于 PDF 采样
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, mat: Arc<dyn Material>) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        let n = Vec3::cross(&edge1, &edge2);       // 未归一化的法向量
        let normal = Vec3::unit_vector(&n);         // 归一化法向量
        let area = 0.5 * n.length();                // 三角形面积 = 0.5 × 两边叉积模长

        // 通过顶点计算 AABB，用于 BVH 包围盒优化
        let bbox1 = Aabb::from_points(v0, v1);
        let bbox2 = Aabb::from_points(v1, v2);
        let bbox = Aabb::surrounding_box(&bbox1, &bbox2);

        Self {
            v0,
            v1,
            v2,
            normal,
            mat,
            bbox,
            area,
        }
    }

    #[warn(dead_code)]
    fn is_inside(&self, u: f64, v: f64, w: f64) -> bool {
        u >= 0.0 && v >= 0.0 && w >= 0.0 && (u + v + w - 1.0).abs() < 1e-6
    }
}

impl Hittable for Triangle {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'a>> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;

        let h = Vec3::cross(r.direction(), &edge2);
        let a = Vec3::dot(&edge1, &h);

        if a.abs() < 1e-8 {
            return None;    // 如果 a 过小，说明光线与三角形平行
        }

        let f = 1.0 / a;
        let s = *r.origin() - self.v0;
        let u = f * Vec3::dot(&s, &h); // 重心坐标 u
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = Vec3::cross(&s, &edge1);
        let v = f * Vec3::dot(r.direction(), &q); // 重心坐标 v
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * Vec3::dot(&edge2, &q); // 相交时间 t
        if !ray_t.contains(t) {
            return None;
        }

        let t = f * Vec3::dot(&edge2, &q); // 相交时间 t
        if !ray_t.contains(t) {
            return None;
        }

        // 计算交点
        let intersection = r.at(t);

        // 构造交点记录
        let mut rec = HitRecord {
            p: intersection,
            normal: Vec3::new(0.0, 0.0, 0.0),
            t,
            front_face: true,
            mat: &*self.mat,
            u,
            v,
        };
        rec.set_face_normal(r, &self.normal);
        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let ray = Ray::new(*origin, *direction, 0.0);
        if let Some(rec) = self.hit(&ray, &Interval::new(0.001, f64::INFINITY)) {
            let distance_squared = rec.t * rec.t * direction.length_squared();
            let cosine = Vec3::dot(direction, &rec.normal).abs() / direction.length();
            return distance_squared / (cosine * self.area);
        }
        0.0
    }

    /// 在三角形表面随机采样一点，返回方向向量
    fn random(&self, origin: &Point3) -> Vec3 {
        // 重心坐标采样公式（均匀）
        let r1 = rtweekend::random_double();
        let r2 = rtweekend::random_double();

        let sqrt_r1 = r1.sqrt();
        let a = 1.0 - sqrt_r1;
        let b = r2 * sqrt_r1;
        let c = 1.0 - a - b;

        let random_point = self.v0 * a + self.v1 * b + self.v2 * c;
        random_point - *origin
    }
}
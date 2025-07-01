use crate::hittable::{Hittable,HitRecord};
use crate::ray::Ray;
use crate::{rtweekend, Arc};
use crate::material::Material;
use crate::vec3::{Vec3,Point3};
use crate::interval::Interval;
use crate::AABB::Aabb;

pub struct Sphere {
    pub center : Ray,
    pub radius : f64,
    mat: Arc<dyn Material>,
    bbox : Aabb,
}

impl Sphere {
    pub fn new_stationary(center1 : Point3, radius1 : f64, mat : Arc<dyn Material>) -> Self {
        let r = radius1.max(0.0);
        let rvec = Vec3::new(r, r, r); // 向量形式的半径
        let bbox = Aabb::from_points(center1 - rvec, center1 + rvec);

        Self {
            center: Ray::from(center1, Vec3::new(0.0, 0.0, 0.0)), // 表示静止球体
            radius: r,
            mat,
            bbox,
        }
    }

    // 创建运动球体
    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let r = radius.max(0.0);
        let rvec = Vec3::new(r, r, r);
        let box1 = Aabb::from_points(center1 - rvec, center1 + rvec);
        let box2 = Aabb::from_points(center2 - rvec, center2 + rvec);
        let bbox = Aabb::surrounding_box(&box1, &box2); // 合并两个时刻的 AABB

        Self {
            center: Ray::from(center1, center2 - center1), // 方向向量代表运动速度
            radius: r,
            mat,
            bbox,
        }
    }

    fn get_sphere_uv(p : &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + rtweekend::PI_F64;

        (phi / (2.0 * rtweekend::PI_F64), theta / rtweekend::PI_F64)
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
        let (u,v) = Self::get_sphere_uv(&outward_normal);
        let mut rec = HitRecord { p, normal : Vec3::new(0.0, 0.0, 0.0), t : root, front_face : true, mat : mat, u : u, v : v };
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
    
    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
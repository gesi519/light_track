use crate::AABB::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::{Arc, rtweekend};
use crate::onb::Onb;

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new_stationary(center1: Point3, radius1: f64, mat: Arc<dyn Material>) -> Self {
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
    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Arc<dyn Material>,
    ) -> Self {
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

    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + rtweekend::PI_F64;

        (phi / (2.0 * rtweekend::PI_F64), theta / rtweekend::PI_F64)
    }

    fn random_to_sphere(radius : f64, distance_square : f64) -> Vec3 {
        let r1 = rtweekend::random_double();
        let r2 = rtweekend::random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_square).sqrt() - 1.0);

        let phi = 2.0 * rtweekend::PI_F64 * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();
        Vec3::new(x, y, z)
    }
}

impl Hittable for Sphere {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'a>> {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);

        let outward_normal = (p - current_center) / self.radius;

        // let mat = Arc::clone(&self.mat);
        let (u, v) = Self::get_sphere_uv(&outward_normal);

        let mut rec = HitRecord {
            p,
            normal: Vec3::new(0.0, 0.0, 0.0), // 后续 set_face_normal 会设定
            t: root,
            front_face: true,
            mat: &*self.mat,
            u,
            v,
        };
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        // 只适用于静止球体
        let r = Ray::new(*origin, *direction, 0.0);
        let interval = Interval::new(0.001, f64::INFINITY);
        if let Some(_rec) = self.hit(&r, &interval) {
            let dist_squared = (self.center.orig - *origin).length_squared();
            let cos_theta_max = (1.0 - self.radius * self.radius / dist_squared).sqrt();
            let solid_angle = 2.0 * rtweekend::INFINITY_F64 * (1.0 - cos_theta_max);

            1.0 / solid_angle
        }else{
            0.0
        }
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let direction = self.center.at(0.0) - *origin;
        let distance_squared = direction.length_squared();
        let uvw = Onb::new(direction);
        uvw.transform(&Sphere::random_to_sphere(self.radius, distance_squared))
    }
}

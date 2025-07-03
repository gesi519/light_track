use crate::AABB::Aabb;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = Vec3::cross(&u, &v);
        let normal = Vec3::unit_vector(&n);
        let d = Vec3::dot(&normal, &q);
        let w = n / Vec3::dot(&n, &n);
        let mut quad = Self {
            q: q,
            u: u,
            v: v,
            mat: mat,
            bbox: Aabb::empty(),
            normal: normal,
            d: d,
            w: w,
        };
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        let p1 = self.q;
        let p2 = self.q + self.u + self.v;
        let p3 = self.q + self.u;
        let p4 = self.q + self.v;

        let bbox1 = Aabb::from_points(p1, p2);
        let bbox2 = Aabb::from_points(p3, p4);
        self.bbox = Aabb::surrounding_box(&bbox1, &bbox2);
    }

    pub fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }

    pub fn make_box(a: &Point3, b: &Point3, mat: Arc<dyn Material>) -> Arc<HittableList> {
        let mut sides = HittableList::new();

        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            mat.clone(),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            mat.clone(),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            mat.clone(),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            mat.clone(),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            mat.clone(),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            mat,
        )));

        Arc::new(sides)
    }
}

impl Hittable for Quad {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'a>> {
        let denom = Vec3::dot(&self.normal, r.direction());

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - Vec3::dot(&self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));

        let mut rec = HitRecord {
            p: intersection,
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: t,
            front_face: true,
            mat: &*self.mat,
            u: 0.0,
            v: 0.0,
        };
        if !self.is_interior(alpha, beta, &mut rec) {
            return None;
        }
        rec.set_face_normal(r, &self.normal);
        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

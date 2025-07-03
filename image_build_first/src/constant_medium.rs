use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::texture::Texture;
use crate::vec3::{Color, Vec3};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new_with_texture(
        boundary: Arc<dyn Hittable>,
        density: f64,
        texture: Arc<dyn Texture>,
    ) -> Self {
        let phase_function = Arc::new(Isotropic::new_with_texture(texture));
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }

    pub fn new_with_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        let phase_function = Arc::new(Isotropic::new_with_color(albedo));
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'a>> {
        let rec1 = self.boundary.hit(r, &Interval::universe())?;
        let rec2 = self
            .boundary
            .hit(r, &Interval::new(rec1.t + 0.0001, f64::INFINITY))?;

        let mut t1 = rec1.t;
        let mut t2 = rec2.t;

        if t1 < ray_t.min {
            t1 = ray_t.min;
        }
        if t2 > ray_t.max {
            t2 = ray_t.max;
        }

        if t1 >= t2 {
            return None;
        }
        if t1 < 0.0 {
            t1 = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (t2 - t1) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = t1 + hit_distance / ray_length;
        let p = r.at(t);

        Some(HitRecord {
            t,
            p,
            normal: Vec3::new(1.0, 0.0, 0.0), // arbitrary
            front_face: true,                 // arbitrary
            mat: &*self.phase_function,
            u: 0.0,
            v: 0.0,
        })
    }

    fn bounding_box(&self) -> crate::AABB::Aabb {
        self.boundary.bounding_box()
    }
}

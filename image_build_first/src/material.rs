use std::fmt::Debug;

use crate::vec3::{Color,Vec3};
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material : Send + Sync + Debug {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo : Color, //  反射率的比例
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter( &self, r_in : &Ray, rec : &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo : Color,
}

impl Metal {
    pub fn new(albedo : Color) -> Metal {
        Metal{albedo}
    } 
}

impl Material for Metal {
    fn scatter( &self, r_in : &Ray, rec : &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(*r_in.direction(), rec.normal);

        let scattered = Ray::new(rec.p, reflected);
        Some((scattered, self.albedo))
    }
}
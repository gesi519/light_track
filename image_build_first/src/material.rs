use std::fmt::Debug;

use crate::rtweekend::random_double;
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
    pub fn new(albedo : Color) -> Self {
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
    pub fuzz : f64,
}

impl Metal {
    pub fn new(albedo : Color, fuzz : f64) -> Metal {
        Metal{albedo, fuzz}
    } 
}

impl Material for Metal {
    fn scatter( &self, r_in : &Ray, rec : &HitRecord) -> Option<(Ray, Color)> {
        let reflected1 = Vec3::reflect(r_in.direction(), &rec.normal);
        let reflected = Vec3::unit_vector(&reflected1) + self.fuzz * Vec3::random_unit_vector(); 
        let scattered = Ray::new(rec.p, reflected);
        if Vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        }else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Dielectric {
    refraction_index : f64,
}

impl Dielectric {
    pub fn new(refraction_index : f64) -> Self {
        Self { refraction_index: refraction_index }
    }

    //  模拟现实中的玻璃材质模拟：随角度变化而折射率变化
    fn reflectance(cosine : f64, refraction_index : f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        }else {
            self.refraction_index
        };
           
        let uint_direction = Vec3::unit_vector(r_in.direction());
        let cos_theta = Vec3::dot(&(-uint_direction), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if ri * sin_theta > 1.0 || Dielectric::reflectance(cos_theta, ri) > random_double() {
            Vec3::reflect(&uint_direction, &rec.normal)
        }else {
            Vec3::refract(&uint_direction, rec.normal, ri)
        };
        
        Some((Ray::new(rec.p, direction),attenuation))
    }
}
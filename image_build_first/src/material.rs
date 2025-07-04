use std::fmt::Debug;
use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::{self, random_double};
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Color, Point3, Vec3};
use crate::onb::Onb;

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        None
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in : &Ray, _rec : &HitRecord, _scattered : &Ray) -> f64 {
        0.0
    }
}

#[derive(Debug)]
pub struct Lambertian {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from_texture(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        let uvw = Onb::new(rec.normal);
        let scatter_direction = uvw.transform(&Vec3::random_cosine_direction());

        // if scatter_direction.near_zero() {
        //     scatter_direction = rec.normal;
        // }

        let scattered = Ray::new(rec.p, Vec3::unit_vector(&scatter_direction), r_in.time());
        let pdf = Vec3::dot(uvw.w(), scattered.direction()) / rtweekend::PI_F64;
        Some((scattered, self.tex.value(rec.u, rec.v, &rec.p), pdf))
    }

    fn scattering_pdf(&self, _r_in : &Ray, _rec : &HitRecord, _scattered : &Ray) -> f64 {
        // let cos_theta = Vec3::dot(&rec.normal, &Vec3::unit_vector(scattered.direction()));
        // if cos_theta < 0.0 {
        //     0.0
        // }else {
        //     cos_theta / rtweekend::PI_F64
        // }
        1.0 / (2.0 * rtweekend::PI_F64)
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        let reflected1 = Vec3::reflect(r_in.direction(), &rec.normal);
        let reflected = Vec3::unit_vector(&reflected1) + self.fuzz * Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, reflected, r_in.time());
        if Vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, self.albedo, 1.0))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index: refraction_index,
        }
    }

    //  模拟现实中的玻璃材质模拟：随角度变化而折射率变化
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let uint_direction = Vec3::unit_vector(r_in.direction());
        let cos_theta = Vec3::dot(&(-uint_direction), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction =
            if ri * sin_theta > 1.0 || Dielectric::reflectance(cos_theta, ri) > random_double() {
                Vec3::reflect(&uint_direction, &rec.normal)
            } else {
                Vec3::refract(&uint_direction, rec.normal, ri)
            };

        Some((Ray::new(rec.p, direction, r_in.time()), attenuation, 1.0))
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(c: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            return Color::new(0.0, 0.0, 0.0);
        }
        self.tex.as_ref().value(u, v, p)
    }
}

#[derive(Debug)]
pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_with_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex: tex }
    }

    pub fn new_with_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        Some((
            Ray::new(rec.p, Vec3::random_unit_vector(), r_in.time()),
            self.tex.value(rec.u, rec.v, &rec.p),
            1.0 / (4.0 * rtweekend::PI_F64),
        ))
    }

    fn scattering_pdf(&self, _r_in : &Ray, _rec : &HitRecord, _scattered : &Ray) -> f64 {
        1.0 / (4.0 * rtweekend::PI_F64)
    }
}

#[derive(Debug)]
pub struct EmptyMaterial;

impl Material for EmptyMaterial {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color, f64)> {
        None
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in : &Ray, _rec : &HitRecord, _scattered : &Ray) -> f64 {
        0.0
    }
}
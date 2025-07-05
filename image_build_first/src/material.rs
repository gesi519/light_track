use std::fmt::Debug;
use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::pdf::{CosinePdf, Pdf, SpherePdf};
use crate::ray::Ray;
use crate::rtweekend::{self, random_double};

use crate::texture::{SolidColor, Texture};
use crate::vec3::{Color, Point3, Vec3};

pub struct  ScatterRecord {
    pub attenuation : Color,
    pub pdf_ptr: Option<Arc<dyn Pdf + Send + Sync>>,
    pub skip_pdf_ray: Option<Ray>,
}

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord)  -> Option<ScatterRecord>  {
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord)  -> Option<ScatterRecord>  {
        Some(ScatterRecord { attenuation: self.tex.value(rec.u, rec.v, &rec.p), pdf_ptr: Some(Arc::new(CosinePdf::new(rec.normal))), skip_pdf_ray: None })
    }

    fn scattering_pdf(&self, _r_in : &Ray, rec : &HitRecord, scattered : &Ray) -> f64 {
        let cos_theta = Vec3::dot(&rec.normal, &Vec3::unit_vector(scattered.direction()));
        if cos_theta < 0.0 {
            0.0
        }else {
            cos_theta / rtweekend::PI_F64
        }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected1 = Vec3::reflect(r_in.direction(), &rec.normal);
        let reflected = Vec3::unit_vector(&reflected1) + self.fuzz * Vec3::random_unit_vector();
        
        Some(ScatterRecord { attenuation: self.albedo, pdf_ptr: None, skip_pdf_ray: Some(Ray::new(rec.p, reflected, r_in.time())) })
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
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

        Some(ScatterRecord { attenuation: Color::new(1.0, 1.0, 1.0), pdf_ptr: None, skip_pdf_ray: Some(Ray::new(rec.p,direction, r_in.time())) })
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord { attenuation: self.tex.value(rec.u, rec.v, &rec.p), pdf_ptr: Some(Arc::new(SpherePdf::new())), skip_pdf_ray: None })
    }

    fn scattering_pdf(&self, _r_in : &Ray, _rec : &HitRecord, _scattered : &Ray) -> f64 {
        1.0 / (4.0 * rtweekend::PI_F64)
    }
}

#[derive(Debug)]
pub struct EmptyMaterial;

impl Material for EmptyMaterial {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in : &Ray, _rec : &HitRecord, _scattered : &Ray) -> f64 {
        0.0
    }
}
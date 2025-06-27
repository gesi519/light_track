
use crate::hittable::{Hittable};
use crate::vec3::{Color,Vec3,Point3};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::rtweekend::{self, random_double};

use std::io::{Write,stderr};

pub struct Camera {
    pub aspect_ratio : f64,
    pub image_width : usize,
    image_height : usize,
    center : Point3,
    pixel00_loc : Point3,   //  定位像素点 0，0
    pixel_delta_u : Vec3,
    pixel_delta_v : Vec3,
    pub sample_per_pixel : usize,
    pixel_samples_scale : f64,
    pub max_depth : usize,
}

impl Camera {
    pub fn ray_color(r : &Ray, world : &dyn Hittable, depth : usize) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(r, &Interval::new(0.001, rtweekend::INFINITY_F64)) {
            if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
                return attenuation * Camera::ray_color(&scattered, world, depth - 1)
            }
            return Color::new(0.0, 0.0, 0.0);
        }
        
        let unit_direction = Vec3::unit_vector(*r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 0,               // 会在 initialize 中计算
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            sample_per_pixel : 10,
            pixel_samples_scale : 0.0,
            max_depth : 10,
        }
    }

    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.pixel_samples_scale = 1.0 / self.sample_per_pixel as f64;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width : f64 = viewport_height * (self.image_width as f64/ self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width,0.0,0.0);
        let viewport_v = Vec3::new(0.0 ,-viewport_height,0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - Vec3::new(0.0,0.0,focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render<W: Write>(&mut self, world : &dyn Hittable, mut writer : W)  -> std::io::Result<()> {
        self.initialize();

        writeln!(writer,"P3")?;
        writeln!(writer,"{} {}", self.image_width, self.image_height)?;
        writeln!(writer,"255")?;


        for j in 0..self.image_height {
            eprint!("Scanlines remaining: {}\n", self.image_height - j);
            stderr().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.sample_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, world, self.max_depth);
                }
                Color::write_color(&mut writer, self.pixel_samples_scale * pixel_color)?;
            }
        }
        eprintln!("Done.                 \n");
        Ok(())
    }

    pub fn get_ray(&self ,i : usize, j : usize) -> Ray {
        let offset : Vec3 = Camera::sample_square();
        let pixel_sample = self.pixel00_loc + 
                                ((i as f64+ offset.x()) * self.pixel_delta_u) + 
                                ((j as f64+ offset.x()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray { orig: ray_origin, dir: ray_direction }
    }

    pub fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
    
}



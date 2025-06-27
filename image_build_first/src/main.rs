//  rust里面没有std::clog,通常使用eprint! 和 eprintln! 向标准错误输出
use std::sync::Arc;
use std::io::{BufWriter, stdout};

pub mod vec3;
use crate::vec3::{Point3, Color};

pub mod hittable;
pub mod rtweekend;
pub mod interval;
pub mod camera;
pub mod ray;
pub mod material;
use crate::camera::Camera;
use crate::material::{Lambertian, Metal, Dielectric};

use crate::hittable::{HittableList,Sphere};


fn main() -> std::io::Result<()> {

    let aspect_ratio : f64 = 16.0 / 9.0;
    let image_width : usize = 400;

    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Arc::new(Dielectric::new(1.00 / 1.33));
    let material_right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2) ,1.0));

    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;

    let stdout = stdout();                      // 获取 stdout 句柄
    let writer = BufWriter::new(stdout); 
    cam.render(&world, writer)?;  
    Ok(())
}



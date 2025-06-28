//  rust里面没有std::clog,通常使用eprint! 和 eprintln! 向标准错误输出
use std::sync::Arc;
use std::io::{BufWriter, stdout};

pub mod vec3;
use crate::vec3::{Point3, Color, Vec3};

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
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new( a as f64 + 0.9 * rtweekend::random_double(), 
                                            0.2, 
                                            b as f64 + 0.9 * rtweekend::random_double());
            
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //  diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }else if choose_mat < 0.95 {
                    //  matal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo,fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }else {
                    //  glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }

        }
    }

    let material1 = Arc::new(Dielectric::new(1.50));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));
    // let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left   = Arc::new(Dielectric::new(1.50));
    // let material_bubble = Arc::new(Dielectric::new( 1.00 / 1.50));
    // let material_right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2) ,1.0));

    // world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    // world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    // world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    // world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    // world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let aspect_ratio : f64 = 16.0 / 9.0;
    let image_width : usize = 1200;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    let stdout = stdout();                      // 获取 stdout 句柄
    let writer = BufWriter::new(stdout); 
    cam.render(&world, writer)?;  
    Ok(())
}



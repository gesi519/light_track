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
pub mod sphere;
pub mod AABB;
pub mod bvh;
pub mod texture;
pub mod rtw_image;
pub mod perlin;
pub mod quad;
use crate::camera::Camera;
use crate::material::{Lambertian, Metal, Dielectric};

use crate::hittable::{HittableList};
use crate::sphere::Sphere;
use crate::bvh::BvhNode;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::quad::Quad;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    // eprintln!("Current dir: {:?}\n", std::env::current_dir().unwrap());
    let start = Instant::now();

    match 5 {
        2 => checker_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        _ => bouncing_spheres(),
    }?;

    let duration = start.elapsed();
    eprintln!("运行时间: {:?}\n", duration);
    Ok(())
}

fn bouncing_spheres() -> Result<(), std::io::Error> {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    world.add(Arc::new(Sphere::new_stationary(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::from_texture(checker)))));
    // let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    // world.add(Arc::new(Sphere::new_stationary(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

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
                    let center2 = center + Vec3::new(0.0, rtweekend::random_double_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(center, center2,0.2, sphere_material)));
                }else if choose_mat < 0.95 {
                    //  matal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo,fuzz));
                    world.add(Arc::new(Sphere::new_stationary(center, 0.2, sphere_material)));
                }else {
                    //  glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new_stationary(center, 0.2, sphere_material)));
                }
            }

        }
    }

    let material1 = Arc::new(Dielectric::new(1.50));
    world.add(Arc::new(Sphere::new_stationary(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new_stationary(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new_stationary(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

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
    cam.initialize();
    cam.render(world, writer)?;  
    Ok(())
}

fn checker_spheres() -> Result<(), std::io::Error> {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    world.add(Arc::new(Sphere::new_stationary(Point3::new(0.0, -10.0, 0.0), 10.0, Arc::new(Lambertian::from_texture(checker.clone())))));
    world.add(Arc::new(Sphere::new_stationary(Point3::new(0.0, 10.0, 0.0), 10.0, Arc::new(Lambertian::from_texture(checker)))));

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

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

    cam.defocus_angle = 0.0;

    let stdout = stdout();                      // 获取 stdout 句柄
    let writer = BufWriter::new(stdout); 
    cam.initialize();
    cam.render(world, writer)?; 
    Ok(())
}

fn earth() -> Result<(), std::io::Error> {
    let mut world = HittableList::new();
    let earth_surface = Arc::new(ImageTexture::new("earthmap.jpg"));
    world.add(Arc::new(Sphere::new_stationary(Point3::new(0.0, 0.0, 0.0), 2.0, Arc::new(Lambertian::from_texture(earth_surface)))));

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio : f64 = 16.0 / 9.0;
    let image_width : usize = 400;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout();                      // 获取 stdout 句柄
    let writer = BufWriter::new(stdout); 
    cam.initialize();
    cam.render(world, writer)?; 
    Ok(())
}

fn perlin_spheres() -> Result<(), std::io::Error> {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new_stationary(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::from_texture(pertext.clone())))));
    world.add(Arc::new(Sphere::new_stationary(Point3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::from_texture(pertext)))));

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio : f64 = 16.0 / 9.0;
    let image_width : usize = 400;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout();                      // 获取 stdout 句柄
    let writer = BufWriter::new(stdout); 
    cam.initialize();
    cam.render(world, writer)?; 
    Ok(())
}

fn quads() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red)));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green)));
    world.add(Arc::new(Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue)));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange)));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), lower_teal)));

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio : f64 = 1.0;
    let image_width : usize = 400;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout();                      // 获取 stdout 句柄
    let writer = BufWriter::new(stdout); 
    cam.initialize();
    cam.render(world, writer)?; 
    Ok(())
}

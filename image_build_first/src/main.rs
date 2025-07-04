//  rust里面没有std::clog,通常使用eprint! 和 eprintln! 向标准错误输出
use std::io::{BufWriter, stdout};
use std::sync::Arc;

pub mod vec3;
use crate::constant_medium::ConstantMedium;
use crate::vec3::{Color, Point3, Vec3};

pub mod AABB;
pub mod bvh;
pub mod camera;
pub mod constant_medium;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod quad;
pub mod ray;
pub mod rtw_image;
pub mod rtweekend;
pub mod sphere;
pub mod texture;
pub mod onb;
pub mod pdf;
use crate::camera::Camera;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal, EmptyMaterial};

use crate::bvh::BvhNode;
use crate::hittable::{HittableList, RotateY, Translate, Hittable};
use crate::quad::Quad;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};

use std::time::Instant;

// use {
//     anyhow::{Context, Result},
//     winit::{
//         event::{Event, WindowEvent},
//         event_loop::{ControlFlow, EventLoop},
//         window::{Window, WindowBuilder},
//     },
// };

// const WIDTH: u32 = 800;
// const HEIGHT: u32 = 600;

use pprof::ProfilerGuard;





fn main() -> std::io::Result<()> {
    // let guard = ProfilerGuard::new(100).unwrap();
    // eprintln!("Current dir: {:?}\n", std::env::current_dir().unwrap());
    let start = Instant::now();
    
    match 7 {
        1 => bouncing_spheres(),
        2 => checker_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        _ => final_scene(400, 250, 4),
    }?;

    let duration = start.elapsed();
    eprintln!("运行时间: {:?}\n", duration);
    // if let Ok(report) = guard.report().build() {
    //     let file = std::fs::File::create("flamegraph.svg").unwrap();
    //     report.flamegraph(file).unwrap();
    // }
    Ok(())
}

fn bouncing_spheres() -> Result<(), std::io::Error> {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(checker)),
    )));
    // let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    // world.add(Arc::new(Sphere::new_stationary(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * rtweekend::random_double(),
                0.2,
                b as f64 + 0.9 * rtweekend::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //  diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 =
                        center + Vec3::new(0.0, rtweekend::random_double_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    //  matal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    //  glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.50));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let empty_material = Arc::new(EmptyMaterial {});
    let quad_lights = Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), empty_material);
    let lights : Arc<dyn Hittable + Send + Sync> = Arc::new(quad_lights);

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 1200;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 500;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    let stdout = stdout(); // 获取 stdout 句柄
    let writer = BufWriter::new(stdout);
    cam.initialize();
    cam.render(world, writer,lights)?;
    Ok(())
}

fn checker_spheres() -> Result<(), std::io::Error> {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(checker)),
    )));

    let empty_material = Arc::new(EmptyMaterial {});
    let quad_lights = Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), empty_material);
    let lights : Arc<dyn Hittable + Send + Sync> = Arc::new(quad_lights);

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 1200;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 500;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout(); // 获取 stdout 句柄
    let writer = BufWriter::new(stdout);
    cam.initialize();
    cam.render(world, writer,lights)?;
    Ok(())
}

fn earth() -> Result<(), std::io::Error> {
    let mut world = HittableList::new();
    let earth_surface = Arc::new(ImageTexture::new("earthmap.jpg"));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(earth_surface)),
    )));

    let empty_material = Arc::new(EmptyMaterial {});
    let quad_lights = Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), empty_material);
    let lights : Arc<dyn Hittable + Send + Sync> = Arc::new(quad_lights);

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout(); // 获取 stdout 句柄
    let writer = BufWriter::new(stdout);
    cam.initialize();
    cam.render(world, writer, lights)?;
    Ok(())
}

fn perlin_spheres() -> Result<(), std::io::Error> {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let empty_material = Arc::new(EmptyMaterial {});
    let quad_lights = Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), empty_material);
    let lights : Arc<dyn Hittable + Send + Sync> = Arc::new(quad_lights);

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout(); // 获取 stdout 句柄
    let writer = BufWriter::new(stdout);
    cam.initialize();
    cam.render(world, writer, lights)?;
    Ok(())
}

fn quads() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let empty_material = Arc::new(EmptyMaterial {});
    let quad_lights = Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), empty_material);
    let lights : Arc<dyn Hittable + Send + Sync> = Arc::new(quad_lights);

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio: f64 = 1.0;
    let image_width: usize = 400;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout(); // 获取 stdout 句柄
    let writer = BufWriter::new(stdout);
    cam.initialize();
    cam.render(world, writer, lights)?;
    Ok(())
}

fn simple_light() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    let empty_material = Arc::new(EmptyMaterial {});
    let quad_lights = Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), empty_material);
    let lights : Arc<dyn Hittable + Send + Sync> = Arc::new(quad_lights);

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
    cam.lookat = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout(); // 获取 stdout 句柄
    let writer = BufWriter::new(stdout);
    cam.initialize();
    cam.render(world, writer, lights)?;
    Ok(())
}

fn cornell_box() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let lignt = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        lignt,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // world.add(Quad::make_box(&Point3::new(130.0, 0.0, 65.0), &Point3::new(295.0, 165.0, 230.0), white.clone()));
    // world.add(Quad::make_box(&Point3::new(265.0, 0.0, 295.0), &Point3::new(430.0, 330.0, 460.0), white));
    let box1 = Quad::make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = Quad::make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let empty_material = Arc::new(EmptyMaterial {});
    let quad_lights = Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), empty_material);
    let lights : Arc<dyn Hittable + Send + Sync> = Arc::new(quad_lights);

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio: f64 = 1.0;
    let image_width: usize = 600;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 1000;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout(); // 获取 stdout 句柄
    let writer = BufWriter::new(stdout);
    cam.initialize();
    cam.render(world, writer, lights)?;
    Ok(())
}

fn cornell_smoke() -> Result<(), std::io::Error> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let lignt = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        lignt,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = Quad::make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstantMedium::new_with_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    let box2 = Quad::make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::new_with_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    let empty_material = Arc::new(EmptyMaterial {});
    let quad_lights = Quad::new(Point3::new(113.0, 554.0, 127.0), Vec3::new(330.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 305.0), empty_material);
    let lights : Arc<dyn Hittable + Send + Sync> = Arc::new(quad_lights);

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio: f64 = 1.0;
    let image_width: usize = 600;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout(); // 获取 stdout 句柄
    let writer = BufWriter::new(stdout);
    cam.initialize();
    cam.render(world, writer, lights)?;
    Ok(())
}

fn final_scene(
    image_width: usize,
    sample_per_pixel: usize,
    max_depth: usize,
) -> Result<(), std::io::Error> {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rtweekend::random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Quad::make_box(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();
    world.add(Arc::new(BvhNode::new_from_list(&boxes1)));

    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new_stationary(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new_with_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new_with_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::from_texture(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    for _j in 0..1000 {
        boxes2.add(Arc::new(Sphere::new_stationary(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_from_list(&boxes2)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let empty_material = Arc::new(EmptyMaterial {});
    let quad_lights = Quad::new(Point3::new(123.0, 554.0, 147.0), Vec3::new(300.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 265.0), empty_material);
    let lights : Arc<dyn Hittable + Send + Sync> = Arc::new(quad_lights);

    let bvh_root = Arc::new(BvhNode::new_from_list(&world));
    let world = bvh_root;

    let aspect_ratio: f64 = 1.0;

    //  camera
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.sample_per_pixel = sample_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let stdout = stdout(); // 获取 stdout 句柄
    let writer = BufWriter::new(stdout);
    cam.initialize();
    cam.render(world, writer, lights)?;
    Ok(())
}

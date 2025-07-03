use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{self, random_double};
use crate::vec3::{Color, Point3, Vec3};

use std::sync::Condvar;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering as AtomicOrdering;
use std::sync::{Arc, Mutex};

use std::io::Write;

const HEIGHT_PARTITION: usize = 32;
const WIDTH_PARTITION: usize = 32;
const THREAD_LIMIT: usize = 24;

#[derive(Clone)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3, //  定位像素点 0，0
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pub sample_per_pixel: usize,
    pixel_samples_scale: f64,
    pub max_depth: usize,
    pub vfov: f64, //  垂直视角
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3, //  相对于相机的“上”方向
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pub defocus_angle: f64, //  每个像素射线的变化角度
    pub focus_dist: f64,    //  从相机观察点到完全对焦平面的距离
    defocus_disk_u: Vec3,   //  散焦圆盘的水平半径
    defocus_disk_v: Vec3,   //  Defocus disk vertical radius
    pub background: Color,  // 场景背景
}

impl Camera {
    pub fn ray_color(r: &Ray, world: &dyn Hittable, depth: usize, background: &Color) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        // if let Some(rec) = world.hit(r, &Interval::new(0.001, rtweekend::INFINITY_F64)) {
        //     if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
        //         return attenuation * Camera::ray_color(&scattered, world, depth - 1);
        //     }
        //     return Color::new(0.0, 0.0, 0.0);
        // }

        // let unit_direction = Vec3::unit_vector(r.direction());
        // let a = 0.5 * (unit_direction.y() + 1.0);
        // Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
        let ray_t = Interval::new(0.001, f64::INFINITY);
        if let Some(rec) = world.hit(r, &ray_t) {
            let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);

            if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
                let p = attenuation.max_component().min(0.95);
                if rtweekend::random_double() < p {
                    let color_from_scatter =
                        attenuation * Camera::ray_color(&scattered, world, depth - 1, background);
                    return color_from_emission + color_from_scatter / p;
                }
            }
            return color_from_emission;
        } else {
            *background
        }
    }

    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 0, // 会在 initialize 中计算
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            sample_per_pixel: 10,
            pixel_samples_scale: 0.0,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
            background: Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.pixel_samples_scale = 1.0 / self.sample_per_pixel as f64;

        self.center = self.lookfrom;

        let theta = rtweekend::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = Vec3::unit_vector(&(self.lookfrom - self.lookat));
        self.u = Vec3::unit_vector(&Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = -viewport_height * self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius =
            self.focus_dist * rtweekend::degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render<W: Write + Send>(
        &self,
        world: Arc<dyn Hittable>,
        mut writer: W,
    ) -> std::io::Result<()> {
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.image_width, self.image_height)?;
        writeln!(writer, "255")?;

        let framebuffer = Arc::new(Mutex::new(vec![
            Color::new(0.0, 0.0, 0.0);
            self.image_width * self.image_height
        ]));

        let width = self.image_width;
        let height = self.image_height;

        let max_depth = self.max_depth;

        let camera_ptr = Arc::new(self.clone());

        let chunk_width = (self.image_width + WIDTH_PARTITION - 1) / WIDTH_PARTITION;
        let chunk_height = (self.image_height + HEIGHT_PARTITION - 1) / HEIGHT_PARTITION;

        // 控制线程数量
        let thread_count = Arc::new(AtomicUsize::new(0));
        let thread_control_mutex = Arc::new(Mutex::new(()));
        let thread_control_cvar = Arc::new(Condvar::new());

        crossbeam::thread::scope(|s| {
            for by in 0..HEIGHT_PARTITION {
                for bx in 0..WIDTH_PARTITION {
                    {
                        let mut lock = thread_control_mutex.lock().unwrap();
                        while thread_count.load(AtomicOrdering::SeqCst) >= THREAD_LIMIT {
                            lock = thread_control_cvar.wait(lock).unwrap();
                        }
                        thread_count.fetch_add(1, AtomicOrdering::SeqCst);
                    }

                    let fb = Arc::clone(&framebuffer);
                    let cam = Arc::clone(&camera_ptr);
                    let world = Arc::clone(&world);

                    let thread_count = Arc::clone(&thread_count);
                    let thread_control_cvar = Arc::clone(&thread_control_cvar);
                    let _thread_control_mutex = Arc::clone(&thread_control_mutex);

                    let x_min = bx * chunk_width;
                    let x_max = ((bx + 1) * chunk_width).min(width);
                    let y_min = by * chunk_height;
                    let y_max = ((by + 1) * chunk_height).min(height);

                    s.spawn(move |_| {
                        let mut local_buffer =
                            vec![Color::new(0.0, 0.0, 0.0); (y_max - y_min) * (x_max - x_min)];
                        for j in y_min..y_max {
                            for i in x_min..x_max {
                                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                                for _sample in 0..cam.sample_per_pixel {
                                    let r = cam.get_ray(i, j);
                                    pixel_color += Camera::ray_color(
                                        &r,
                                        world.as_ref(),
                                        max_depth,
                                        &self.background,
                                    );
                                }
                                let idx = (j - y_min) * (x_max - x_min) + (i - x_min);
                                local_buffer[idx] = cam.pixel_samples_scale * pixel_color;
                            }
                        }
                        let mut fb_locked = fb.lock().unwrap();
                        for y in y_min..y_max {
                            for x in x_min..x_max {
                                let idx = (y - y_min) * (x_max - x_min) + (x - x_min);
                                fb_locked[y * width + x] = local_buffer[idx];
                            }
                        }
                        thread_count.fetch_sub(1, AtomicOrdering::SeqCst);
                        thread_control_cvar.notify_one();
                    });
                }
            }
        })
        .unwrap();

        for j in 0..height {
            for i in 0..width {
                let fb = framebuffer.lock().unwrap();
                Color::write_color(&mut writer, &fb[j * width + i])?;
            }
        }

        eprintln!("Done.                 \n");
        Ok(())
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        // 构造一条相机射线，起点位于散焦圆盘上，方向指向像素位置 i，j 附近随机采样的点。
        let offset: Vec3 = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            Camera::disk_sample(&self)
        };
        let ray_direction = pixel_sample - ray_origin;

        let ray_time = rtweekend::random_double();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    pub fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    pub fn disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p[0] * self.defocus_disk_u + p[1] * self.defocus_disk_v
    }
}

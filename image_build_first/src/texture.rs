use std::sync::Arc;
use std::fmt::Debug;

use crate::vec3::{Color, Point3};
use crate::rtw_image::RtwImage;
use crate::interval::Interval;
use crate::perlin::Perlin;

pub trait Texture : Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Debug)]
pub struct SolidColor {
    albedo : Color,
}

impl SolidColor {
    pub fn new(color : Color) ->Self {
        Self { albedo: color }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self { albedo: Color::new(r, g, b) }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    inv_scale : f64,
    even : Arc<dyn Texture + Send + Sync>,
    odd : Arc<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new(
        scale : f64,
        even : Arc<dyn Texture + Send + Sync>,
        odd : Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        let even = Arc::new(SolidColor::new(c1));
        let odd = Arc::new(SolidColor::new(c2));
        Self::new(scale, even, odd)
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        }else {
            self.odd.value(u, v, p)
        }
    }
}

#[derive(Debug)]
pub struct ImageTexture {
    image : RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self { image: RtwImage::new(filename) }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(color_scale * pixel[0] as f64, color_scale * pixel[1] as f64, color_scale * pixel[2] as f64)
    }
}

#[derive(Debug)]
pub struct NoiseTexture {
    noise : Perlin,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self { noise : Perlin::new() }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
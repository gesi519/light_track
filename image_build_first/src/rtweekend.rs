use std::f64::consts::PI;
use rand::Rng;

// 常量定义
pub const INFINITY_F64: f64 = f64::INFINITY;
pub const PI_F64: f64 = PI;

// 工具函数
pub fn degrees_to_radians(degrees : f64) -> f64 {
    degrees * PI / 180.0
}

// 重新导出其他模块里用到的类型
pub use crate::vec3::{Color, Point3, Vec3};

// 生成[0,1)之间的随机浮点数
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.r#gen::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_int(min : usize, max : usize) -> usize {
    random_double_range(min as f64, max as f64 + 1.0) as usize
}
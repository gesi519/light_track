//! # `interval.rs` 模块说明
//!
//! 该模块定义了一个实数区间 Interval 类型，用于表示数值范围 `[min, max]`。
//!
//! 区间广泛用于图形学中的各种计算中，比如：
//! - 光线的合法交点范围（如 t ∈ [0.001, ∞)）
//! - 颜色分量的范围约束（如 RGB ∈ [0.0, 1.0]）
//! - 值的裁剪（clamp）与限制
//!
//! 提供了以下功能：
//! - 创建常规区间、空区间与无限区间
//! - 判断是否包含某个值
//! - 判断是否“严格包含”某值
//! - 将值裁剪（clamp）到区间范围内
//! - 获取区间大小
//!

use crate::{rtweekend};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    pub min : f64,
    pub max : f64,
}

impl Interval {
    pub fn new(min : f64, max : f64) -> Self {
        Self {  min : min,
                max : max
        }
    }

    pub const fn empty() -> Self  {
        Self {  min : rtweekend::INFINITY_F64,
                max : -rtweekend::INFINITY_F64
        }
    }

    pub const fn universe() -> Self {
        Self {  min : -rtweekend::INFINITY_F64,
                max : rtweekend::INFINITY_F64

        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x : f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x : f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x : f64) -> f64 {
        if x < self.min {
            self.min
        }else if x > self.max {
            self.max
        }else {
            x
        }
    }

    pub fn expand(&self, delta : f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }

    pub fn from_two(a: &Interval, b: &Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
}

use std::ops::Add;

impl Add<Interval> for f64 {
    type Output = Interval;
    fn add(self, rhs : Interval) -> Self::Output {
        Interval {
            min : self + rhs.min,
            max : self + rhs.max,
        }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;
    fn add(self, rhs : f64) -> Self::Output {
        Interval {
            min : self.min + rhs,
            max : self.max + rhs,
        }
    }
}
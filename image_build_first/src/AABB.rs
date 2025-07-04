use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct Aabb {
    pub x : Interval,
    pub y : Interval,
    pub z : Interval,
}

impl Aabb {
    pub fn new(x : Interval, y : Interval, z : Interval,) -> Self {
        let mut aabb_sort = Self {x, y, z : z};
        aabb_sort.pad_to_minimums();
        aabb_sort
    }

    pub fn new_empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a.x() <= b.x() {
            Interval::new(a.x(), b.x())
        } else {
            Interval::new(b.x(), a.x())
        };
        let y = if a.y() <= b.y() {
            Interval::new(a.y(), b.y())
        } else {
            Interval::new(b.y(), a.y())
        };
        let z = if a.z() <= b.z() {
            Interval::new(a.z(), b.z())
        } else {
            Interval::new(b.z(), a.z())
        };
        let mut aabb = Self { x, y, z };
        aabb.pad_to_minimums();
        aabb
    }

    pub fn surrounding_box(box0 : &Aabb, box1 : &Aabb) -> Self {
        Self {
            x: Interval::new(
                box0.x.min.min(box1.x.min),
                box0.x.max.max(box1.x.max),
            ),
            y: Interval::new(
                box0.y.min.min(box1.y.min),
                box0.y.max.max(box1.y.max),
            ),
            z: Interval::new(
                box0.z.min.min(box1.z.min),
                box0.z.max.max(box1.z.max),
            ),
        }
    }

     /// 获取对应轴的区间（0: x, 1: y, 2: z）。
    pub fn axis_interval(&self, axis: usize) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis index: {}", axis),
        }
    }

    pub fn hit(&self, r : &Ray, ray_t : &Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();
        let mut t = *ray_t;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let mut t0 = (ax.min - ray_orig[axis]) * adinv;
            let mut t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t.min = t.min.max(t0);
            t.max = t.max.min(t1);

            if t.max <= t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();
        if x_size > y_size {
            if x_size > z_size {
                0
            } else {
                2
            }
        }else {
            if y_size > z_size {
                1
            } else {
                2
            }
        }
    }

    fn pad_to_minimums(&mut self) {
        let delta : f64 = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub const fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub const fn universe() -> Self {
        Self {
            x: Interval::universe(),
            y: Interval::universe(),
            z: Interval::universe(),
        }
    }
}

use std::ops::Add;

impl Add<Vec3> for Aabb {
    type Output = Aabb;
    fn add(self, rhs : Vec3) -> Self::Output {
        Aabb {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;
    fn add(self, rhs : Aabb) -> Self::Output {
        rhs + self
    }
}
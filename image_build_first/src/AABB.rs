use crate::interval::Interval;
use crate::vec3::{Point3};
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct Aabb {
    x : Interval,
    y : Interval,
    z : Interval,
}

impl Aabb {
    pub fn new(x : Interval, y : Interval, z : Interval,) -> Self {
        Self {x, y, z : z}
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
        Self { x, y, z }
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
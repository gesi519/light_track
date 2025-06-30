use crate::AABB::Aabb;
use crate::hittable::{Hittable, HittableList, HitRecord};
use std::sync::Arc;
use crate::rtweekend;
use crate::ray::Ray;
use crate::interval::Interval;

pub struct BvhNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    pub bbox: Aabb,
}

impl BvhNode {
    pub fn new_from_list(list : &HittableList) -> Self {
        let mut objects = list.objects.clone();
        let len = objects.len();
        Self::new(&mut objects, 0, len)
    }

    pub fn new(
        objects: &mut [Arc<dyn Hittable + Send + Sync>],
        start: usize,
        end: usize,
    ) -> Self {
        let mut bbox = Aabb::empty();

        // 计算当前范围内所有对象的包围盒的包围盒（合并包围盒）
        for i in start..end {
            bbox = Aabb::surrounding_box(&bbox, &objects[i].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = end - start;

        let (left, right) : (Arc<dyn Hittable + Send + Sync>, Arc<dyn Hittable + Send + Sync>) =
            if object_span == 1 {
                let object = Arc::clone(&objects[start]);
                (Arc::clone(&object), object)
            }else if object_span == 2 {
                let left = Arc::clone(&objects[start]);
                let right = Arc::clone(&objects[start + 1]);
                (left, right)
            }else {
                objects[start..end].sort_by(|a, b| comparator(a, b));
                let mid = start + object_span / 2;
                let left = Arc::new(Self::new(objects, start, mid));
                let right = Arc::new(Self::new(objects, mid, end));
                (left, right)
            };

            bbox = Aabb::surrounding_box(&left.bounding_box(), &right.bounding_box());

            Self { left, right, bbox }
    }

    fn box_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
        axis: usize,
    ) -> std::cmp::Ordering {
        let a_interval = a.bounding_box().axis_interval(axis);
        let b_interval = b.bounding_box().axis_interval(axis);

        a_interval.min.partial_cmp(&b_interval.min).unwrap_or(std::cmp::Ordering::Equal)
    }

    /// x 轴排序
    pub fn box_x_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
    ) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    /// y 轴排序
    pub fn box_y_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
    ) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    /// z 轴排序
    pub fn box_z_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
    ) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);
        let hit_right = match &hit_left {
            Some(rec) => self.right.hit(r, &Interval::new(ray_t.min, rec.t)),
            None => self.right.hit(r, ray_t),
        };

        match (hit_left, hit_right) {
            (Some(left_rec), Some(right_rec)) => {
                if left_rec.t < right_rec.t {
                    Some(left_rec)
                } else {
                    Some(right_rec)
                }
            }
            (Some(left_rec), None) => Some(left_rec),
            (None, Some(right_rec)) => Some(right_rec),
            (None, None) => None,
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
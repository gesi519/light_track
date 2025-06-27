use crate::rtweekend;


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

    pub fn empty() -> Self  {
        Self {  min : rtweekend::INFINITY_F64,
                max : -rtweekend::INFINITY_F64
        }
    }

    pub fn universe() -> Self {
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
}


use crate::vec3::Vec3;

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new(n: Vec3) -> Self {
        let w = Vec3::unit_vector(&n);
        // 如果w的x分量的绝对值大于0.9，就用(0,1,0)，否则用(1,0,0)
        let a = if w.x().abs() > 0.9 { 
            Vec3::new(0.0, 1.0, 0.0) 
        }else { 
            Vec3::new(1.0, 0.0, 0.0) 
        };
        let v = Vec3::unit_vector(&Vec3::cross(&w, &a));
        let u = Vec3::cross(&w, &v);
        Self {
            axis: [u, v, w],
        }
    }

    pub fn u(&self) -> &Vec3 {
        &self.axis[0]
    }

    pub fn v(&self) -> &Vec3 {
        &self.axis[1]
    }

    pub fn w(&self) -> &Vec3 {
        &self.axis[2]
    }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        v[0] * self.axis[0] + v[1] * self.axis[1] + v[2] * self.axis[2]
    }
}
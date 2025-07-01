use crate::rtweekend;
use crate::vec3::Point3;

const POINT_COUNT : usize = 256;

#[derive(Debug)]
pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    

    pub fn new() -> Self {
        let mut randfloat = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            randfloat[i] = rtweekend::random_double();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x()) as usize & 255;
        let j = (4.0 * p.y()) as usize & 255;
        let k = (4.0 * p.z()) as usize & 255;

        let idx = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        self.randfloat[idx]
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let target = rtweekend::random_int(0, i);
            p.swap(i, target);
        }
    }
}
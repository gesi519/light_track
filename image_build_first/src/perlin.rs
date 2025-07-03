use crate::rtweekend;
use crate::vec3::{Point3, Vec3};

const POINT_COUNT: usize = 256;

#[derive(Debug)]
pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randvec = [Vec3::new(0.0, 0.0, 0.0); POINT_COUNT];
        for i in 0..POINT_COUNT {
            randvec[i] = Vec3::unit_vector(&Vec3::random_range(-1.0, 1.0));
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        // let i = ((4.0 * p.x()) as i32 & 255) as usize;
        // let j = ((4.0 * p.y()) as i32 & 255) as usize;
        // let k = ((4.0 * p.z()) as i32 & 255) as usize;

        // let idx = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        // self.randfloat[idx]
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        // u = u*u*(3.0 - 2.0 * u);
        // v = v*v*(3.0 - 2.0 * v);
        // w = w*w*(3.0 - 2.0 * w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.randvec[idx];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
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

    // fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    //     let mut accum = 0.0;
    //     for i in 0..2 {
    //         let i_f = i as f64;
    //         let weight_u = i_f * u + (1.0 - i_f) * (1.0 - u);
    //         for j in 0..2 {
    //             let j_f = j as f64;
    //             let weight_v = j_f * v + (1.0 - j_f) * (1.0 - v);
    //             for k in 0..2 {
    //                 let k_f = k as f64;
    //                 let weight_w = k_f * w + (1.0 - k_f) * (1.0 - w);
    //                 accum += weight_u * weight_v * weight_w * c[i][j][k];
    //             }
    //         }
    //     }
    //     accum
    // }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            let i_f = i as f64;
            for j in 0..2 {
                let j_f = j as f64;
                for k in 0..2 {
                    let k_f = k as f64;
                    let weight_v = Vec3::new(u - i_f, v - j_f, w - k_f);
                    let dot_product = Vec3::dot(&c[i][j][k], &weight_v);
                    let blend_i = i_f * uu + (1.0 - i_f) * (1.0 - uu);
                    let blend_j = j_f * vv + (1.0 - j_f) * (1.0 - vv);
                    let blend_k = k_f * ww + (1.0 - k_f) * (1.0 - ww);
                    accum += blend_i * blend_j * blend_k * dot_product;
                }
            }
        }
        accum
    }

    pub fn trub(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}

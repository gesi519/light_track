use std::fmt;
use std::ops::{Add, AddAssign, Sub, Neg, Mul, MulAssign, Div, DivAssign, Index, IndexMut};
use std::io::{Write,Result};
use crate::interval::Interval;
use crate::rtweekend;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e : [f64;3],
}

impl Vec3 {
    pub fn new(e0 : f64, e1 : f64, e2 : f64) -> Self{
        Self { e : [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(u : Vec3, v : Vec3) -> f64 {
        u.e[0]*v.e[0] + u.e[1]*v.e[1] + u.e[2]*v.e[2]
    }

    pub fn cross(u : Vec3, v : Vec3) -> Vec3 {
        Vec3::new(u.e[1] * v.e[2] - u.e[2] * u.e[1],
                  u.e[2] * v.e[0] - u.e[0] * u.e[2],
                  u.e[0] * v.e[1] - u.e[1] * u.e[0],
            )
    }

    pub fn unit_vector(v : Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn reflect(v : Vec3, n : Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v,n) * n
    }

    //  计算光线折射，uv射入地方单位向量，n为法线，etai——zheshelv
    pub fn refract(uv : &Vec3, n : Vec3, etai_over_etat : f64) -> Vec3 {
        let cos_theta = Vec3::dot(-*uv, n).min(1.0);    //  确保不会超过 1.0（由于浮点误差）
        let r_out_perp = etai_over_etat * (*uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn random() -> Vec3 {
        Vec3::new(rtweekend::random_double(), rtweekend::random_double(), rtweekend::random_double())
    } 

    pub fn random_range(min : f64, max : f64) -> Vec3 {
        Vec3::new(rtweekend::random_double_range(min,max), rtweekend::random_double_range(min,max), rtweekend::random_double_range(min,max))
    } 

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq;
            }
        }
    }

    //  返回一个在给定法线 normal 所指半球上的随机单位向量
    pub fn random_on_hemisphere(normal : &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_range(-1.0, 1.0);
        if Vec3::dot(on_unit_sphere, *normal) > 0.0 {
            on_unit_sphere
        }else {
            -on_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

}

// 类型别名
pub type Point3 = Vec3;

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.e[0],-self.e[1],-self.e[2])
    }
}

//  +=
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

//  *=
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

//  /=
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self {
        Self::new(self.e[0] + other.e[0],
                  self.e[1] + other.e[1],
                  self.e[2] + other.e[2])
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.e[0] - other.e[0],
                  self.e[1] - other.e[1],
                  self.e[2] - other.e[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self::new(self.e[0]*t, self.e[1]*t, self.e[2]*t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self*v.e[0], self*v.e[1], self*v.e[2])
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2])
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self {
        (1.0 / t) * self
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

pub type Color = Vec3;
impl Color {
    pub fn double_linear_to_gamma(linear_component : f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        }else {
            0.0
        }
    }

    pub fn write_color<W : Write>(writer : &mut W, pixel_color : Color) -> Result<()> {
        let mut r = pixel_color.x();
        let mut g = pixel_color.y();
        let mut b = pixel_color.z();

        r = Color::double_linear_to_gamma(r);
        g = Color::double_linear_to_gamma(g);
        b = Color::double_linear_to_gamma(b);

        let intensity = Interval::new(0.0, 0.999);

        let ir = (256.0 * intensity.clamp(r)) as u8;
        let ig = (256.0 * intensity.clamp(g)) as u8;
        let ib = (256.0 * intensity.clamp(b)) as u8;

        writeln!(writer, "{} {} {}", ir, ig, ib)
    }
}






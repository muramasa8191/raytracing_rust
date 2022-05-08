use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign, Neg};
use crate::core;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z, 
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) ->Vec3 {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        self * (1.0 / rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: - self.x,
            y: - self.y,
            z: - self.z,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }

    pub fn random_vec3() -> Vec3 {
        Vec3::new(core::random_f64(), core::random_f64(), core::random_f64())
    }

    pub fn random_range_vec3(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            core::random_range_f64(min, max),
            core::random_range_f64(min, max),
            core::random_range_f64(min, max)
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range_vec3(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    // pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    //     let in_unit_sphere = random_unit_vector();
    //     if (Vec3::dot(in_unit_sphere, normal) > 0.0) {
    //         in_unit_sphere
    //     } else {
    //         -in_unit_sphere
    //     }
    // }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z < s
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_unit_vector_test() {
        let v1 = Vec3::random_unit_vector();

        assert!(v1.length_squared() < 1.0);
    }

    #[test]
    fn add_test() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(0.5, -1.5, 3.5);
        assert_eq!(v1 + v2, Vec3::new(1.5, 0.5, 6.5));
    }

    #[test]
    fn sub_test() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.4, 1.3, 3.0);

        assert_eq!(v1 - v2, Vec3::new(-1.4, 0.7, 0.0));
    }

    #[test]
    fn mul_test1() {
        let v1 = Vec3::new(1.0, 2.0, -3.0);
        let v2 = Vec3::new(4.0, 5.0, -6.0);

        assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn mul_test2() {
        let v1 = Vec3::new(1.0, 2.0, -3.0);

        assert_eq!(v1 * 3.0, Vec3::new(3.0, 6.0, -9.0));
    }

    #[test]
    fn mul_test3() {
        let v1 = Vec3::new(1.0, 2.0, -3.0);

        assert_eq!(3.0 * v1, Vec3::new(3.0, 6.0, -9.0));
    }

    #[test]
    fn div_test() {
        let v1 = Vec3::new(1.0, 2.0, -3.0);

        assert_eq!(v1 / 2.0, Vec3::new(0.5, 1.0, -1.5));
    }

    #[test]
    fn add_assign_test() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(0.5, -3.0, 10.0);
        v1 += v2;

        assert_eq!(v1, Vec3::new(1.5, -1.0, 13.0));
    }

    #[test]
    fn mul_assign_test() {
        let mut v1 = Vec3::new(1.0, 2.0, -3.0);
        v1 *= 2.0;

        assert_eq!(v1, Vec3::new(2.0, 4.0, -6.0));
    }

    #[test]
    fn div_assign_test() {
        let mut v1 = Vec3::new(1.0, 2.0, -3.0);
        v1 /= 2.0;

        assert_eq!(v1, Vec3::new(0.5, 1.0, -1.5));
    }

    #[test]
    fn neg_test() {
        let v1 = Vec3::new(1.0, -2.0, 3.5);

        assert_eq!(-v1, Vec3::new(-1.0, 2.0, -3.5));
    }

    #[test]
    fn length_squared_test() {
        let v1 = Vec3::new(1.0, 2.0, -3.0);

        assert_eq!(v1.length_squared(), 14.0);
    }

    #[test]
    fn length_test() {
        let v1 = Vec3::new(f64::sqrt(1.3), f64::sqrt(2.2), f64::sqrt(5.5));

        assert_eq!(v1.length(), 3.0);
    }

    #[test]
    fn dot_test() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(Vec3::dot(v1, v2), 32.0);
    }

    #[test]
    fn cross_test() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(Vec3::cross(v1, v2), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn unit_vector_test() {
        let x = f64::sqrt(1.2);
        let y = f64::sqrt(2.1);
        let z = f64::sqrt(5.7);
        let v1 = Vec3::new(x, y, z);

        let l = v1.length();
        assert_eq!(v1.unit_vector(), v1 / l);
    }
}

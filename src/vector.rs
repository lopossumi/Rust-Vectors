use core::fmt;
use std::ops::{Add, Sub, Neg, Mul};

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3{
            x: -self.x,
            y: -self.y, 
            z: -self.z
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3{
            x: self * other.x,
            y: self * other.y, 
            z: self * other.z
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::*;
    use super::*;
    
    macro_rules! assert_vec3_equal {
        ($expected:expr, $actual:expr) => {
            let tolerance = 0.0001;
            assert_approx_eq!($expected.x, $actual.x, tolerance);
            assert_approx_eq!($expected.y, $actual.y, tolerance);
            assert_approx_eq!($expected.z, $actual.z, tolerance);
        }
    }
    
    #[test]
    fn addition() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        let actual = vector1 + vector2;
        let expected = Vec3::new(5.0, 7.0, 9.0);
        assert_vec3_equal!(expected, actual);
    }

    #[test]
    fn substraction() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        let actual = vector1 - vector2;
        let expected = Vec3::new(-3.0, -3.0, -3.0);
        assert_vec3_equal!(expected, actual);
    }

    #[test]
    fn negation() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let actual = -vector1; 
        let expected = Vec3::new(-1.0, -2.0, -3.0);
        assert_vec3_equal!(expected, actual);
    }

    #[test]
    fn multiplication() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let actual = 5.0 * vector1;
        let expected = Vec3::new(5.0, 10.0, 15.0);
        assert_vec3_equal!(expected, actual);
    }
}

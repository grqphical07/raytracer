use std::{ops::{self, Range}, fmt::Display};

use crate::random::{random_f64, random_f64_in_range};

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<Vector3> for usize {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self as f64 * rhs.x,
            y: self as f64 * rhs.y,
            z: self as f64 * rhs.z,
        }
    }
}

impl ops::Mul<Vector3> for i64 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self as f64 * rhs.x,
            y: self as f64 * rhs.y,
            z: self as f64 * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        return (1.0 / rhs) * self
    }
}

impl Vector3 {
    /// Creates an empty Vector
    pub const fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    /// Creates a Vector from a set of floats
    /// 
    /// ## Arguments
    /// - `x` X Coordinate of the Vector
    /// - `y` Y Coordinate of the Vector
    /// - `z` Z Coordinate of the Vector
    pub const fn from(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z
        }
    }
    
    /// Returns a vector with random values between 0 and 1
    pub fn random() -> Self {
        Self {
            x: random_f64(),
            y: random_f64(),
            z: random_f64()
        }
    }
    
    /// Returns a random vector within the given range
    ///
    /// ## Arguments
    /// - `range` Range of numbers to generate within
    pub fn random_in_range(range: Range<f64>) -> Self {
        Self {
            x: random_f64_in_range(range.clone()),
            y: random_f64_in_range(range.clone()),
            z: random_f64_in_range(range)
        }
    }

    /// Returns the length of the Vector
    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt()
    }
    
    /// Returns the length of the vector squared
    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    
    /// Returns the vector's unit
    pub fn unit(&self) -> Self {
        return *self / self.length();
    }
    
    /// Checks if the vector is near zero
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s);
    }
}

/// Finds the dot product of two Vectors
/// 
/// ## Arguments
/// - `a` The First Vector of the product
/// - `b` The Second Vector of the product
pub fn dot_product(a: Vector3, b: Vector3) -> f64 {
    return a.x * b.x + a.y * b.y + a.z * b.z 
}

/// Finds the cross product of two Vectors
/// 
/// ## Arguments
/// - `a` The First Vector of the product
/// - `b` The Second Vector of the product
pub fn cross_product(a: Vector3, b: Vector3) -> Vector3 {
    return Vector3 { x: a.y * b.z - a.z * b.y, y: a.z * b.x - a.x * b.z , z: a.x * b.y - a.y * b.x }
}

/// Returns a random vector within a unit sphere
pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::random_in_range(-1.0..1.0);

            if p.length_squared() < 1.0 {
                return p;
            }
        }
}

/// Returns a random unit sphere's unit
pub fn random_unit_vector() -> Vector3 {
    return random_in_unit_sphere().unit();
}

/// Finds a random point in a hemisphere
///
/// ## Arguments
///
/// - `normal` The normal of the hemisphere
pub fn random_on_hemisphere(normal: Vector3) -> Vector3 {
    let on_unit_sphere = random_unit_vector();
    if dot_product(on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere
    }
}

/// Reflects a vector and a normal
pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    return v - 2.0 * dot_product(v, n) * n;
}


pub fn refract(uv: Vector3, n: Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = dot_product(-uv, n).min(1.0);

    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;

    return r_out_perp + r_out_parallel;
}

pub fn random_in_unit_disk() -> Vector3 {
    loop {
        let p = Vector3::from(random_f64_in_range(-1.0..1.0), random_f64_in_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

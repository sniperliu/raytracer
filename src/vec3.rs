use rand::distributions::{Distribution, Standard};
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn normalize(&self) -> Self {
        assert_ne!(0., self.length());

        self.clone() / self.length()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, v: Self) -> Self {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, v: Self) -> Self {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, t: f32) -> Self::Output {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        }
    }
}

// TODO use macro to implement it
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self::Output {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R>(&self, rng : &mut R) -> Vec3 where R: rand::Rng + ?Sized {
        Vec3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const ORIGIN: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    #[test]
    fn test_new() {
        let v1 = Vec3::new(0., 0., 0.);
        let v2 = Vec3::new(0., 0., 0.);
        assert_eq!(v1, v1);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", ORIGIN), "0 0 0");
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(0., 0., 0.);
        v1 += Vec3::new(1., 0., 0.);
        assert_eq!(Vec3::new(1., 0., 0.), v1);
    }

    #[test]
    fn test_scalar_mul_assign() {
        let mut v1 = Vec3::new(2., 0., 0.);
        v1 *= 2.;
        assert_eq!(Vec3::new(4., 0., 0.), v1);
    }

    #[test]
    fn test_scalar_div_assign() {
        let mut v1 = Vec3::new(2., 0., 0.);
        v1 /= 2.;
        assert_eq!(Vec3::new(1., 0., 0.), v1);
    }

    #[test]
    fn test_negate() {
        assert_eq!(Vec3::new(-1., -1., -1.), Vec3::new(1., 1., 1.).neg());

        assert_eq!(Vec3::new(1., 1., 1.), Vec3::new(-1., -1., -1.).neg());

        assert_eq!(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.).neg());
    }

    #[test]
    fn test_add() {
        assert_eq!(Vec3::new(2., 3., 4.), Vec3::new(2., 3., 4.) + ORIGIN);
        assert_eq!(
            Vec3::new(4., 6., 8.),
            Vec3::new(2., 3., 4.) + Vec3::new(2., 3., 4.)
        );

        let v1 = Vec3::new(2., 3., 4.);
        assert_eq!(Vec3::new(4., 6., 8.), v1 + v1);
    }

    #[test]
    fn test_mul() {
        assert_eq!(ORIGIN, ORIGIN * Vec3::new(1., 2., 3.));
    }

    #[test]
    fn test_scalar_mul() {
        assert_eq!(ORIGIN, ORIGIN * 5.);
        assert_eq!(ORIGIN, 5. * ORIGIN);

        assert_eq!(Vec3::new(5., 5., 5.), Vec3::new(1., 1., 1.) * 5.);
        assert_eq!(Vec3::new(5., 5., 5.), 5. * Vec3::new(1., 1., 1.));
    }

    #[test]
    fn test_scalar_div() {
        assert_eq!(ORIGIN, ORIGIN / 3.);

        let v1 = Vec3::new(1., 1., 1.);
        assert_eq!(Vec3::new(0.5, 0.5, 0.5), v1 / 2.);
    }

    #[test]
    fn test_length_squared() {
        assert_eq!(0., ORIGIN.length_squared());

        assert_eq!(1., Vec3::new(1., 0., 0.).length_squared());
    }

    #[test]
    fn test_length() {
        assert_eq!(0., ORIGIN.length());

        assert_eq!(1., Vec3::new(1., 0., 0.).length());
        assert_eq!(
            Vec3::new(1., 0., 0.).length_squared().sqrt(),
            Vec3::new(1., 0., 0.).length()
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(0., ORIGIN.dot(ORIGIN));

        assert_eq!(3., Vec3::new(1., 1., 1.).dot(Vec3::new(1., 1., 1.)));
    }

    #[test]
    fn test_cross() {
        assert_eq!(ORIGIN, ORIGIN.cross(ORIGIN));

        assert_eq!(
            Vec3::new(0., 0., 1.),
            Vec3::new(1., 0., 0.).cross(Vec3::new(0., 1., 0.))
        );
    }

    #[test]
    fn test_normalize() {
        assert_eq!(Vec3::new(1., 0., 0.), Vec3::new(1., 0., 0.).normalize())
    }
}

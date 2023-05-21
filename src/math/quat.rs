use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub(crate) struct Quat {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

impl Quat {
    pub(crate) fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Quat { w, x, y, z }
    }

    pub(crate) fn identity() -> Self {
        Quat::new(1.0, 0.0, 0.0, 0.0)
    }

    pub(crate) fn conjugate(&self) -> Self {
        Quat::new(self.w, -self.x, -self.y, -self.z)
    }

    fn norm(&self) -> f64 {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub(crate) fn normalize(&mut self) {
        let norm = self.norm().sqrt();
        self.w /= norm;
        self.x /= norm;
        self.y /= norm;
        self.z /= norm;
    }
}

impl Add for Quat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Quat::new(self.w + other.w, self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Quat {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Quat::new(self.w - other.w, self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let w = self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z;
        let x = self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y;
        let y = self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x;
        let z = self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w;

        Quat::new(w, x, y, z)
    }
}
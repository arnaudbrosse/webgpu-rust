use crate::math::vector3::Vector3;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quaternion { x, y, z, w }
    }

    pub fn identity() -> Self {
        Quaternion::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn from_axis_angle(axis: &Vector3, angle: f32) -> Self {
        let half_angle = angle * 0.5;
        let sin_half_angle = f32::sin(half_angle);
        let cos_half_angle = f32::cos(half_angle);

        let scaled_axis = axis.normalized() * sin_half_angle;
        Quaternion::new(scaled_axis.x, scaled_axis.y, scaled_axis.z, cos_half_angle)
    }

    pub fn normalized(&self) -> Self {
        let length = self.length();
        Quaternion::new(
            self.x / length,
            self.y / length,
            self.z / length,
            self.w / length,
        )
    }

    pub fn conjugate(&self) -> Self {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
    }

    pub fn dot(&self, other: &Quaternion) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn multiply(&self, other: &Quaternion) -> Self {
        Quaternion::new(
            self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        )
    }

    pub fn rotate_vector(&self, vector: &Vector3) -> Vector3 {
        let quaternion_vector = Quaternion::new(vector.x, vector.y, vector.z, 0.0);
        let rotated_quaternion = self
            .multiply(&quaternion_vector)
            .multiply(&self.conjugate());
        Vector3::new(
            rotated_quaternion.x,
            rotated_quaternion.y,
            rotated_quaternion.z,
        )
    }
}

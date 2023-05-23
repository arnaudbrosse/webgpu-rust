use crate::math::vector3::Vector3;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    pub elements: [f32; 16],
}

impl Matrix4 {
    pub fn new() -> Self {
        Matrix4 {
            elements: [0.0; 16],
        }
    }

    pub fn identity() -> Self {
        let mut matrix = Matrix4::new();
        matrix.elements[0] = 1.0;
        matrix.elements[5] = 1.0;
        matrix.elements[10] = 1.0;
        matrix.elements[15] = 1.0;
        // to finish
    }

    pub fn translate(&mut self, translation: &Vector3) {
        self.elements[12] += translation.x;
        self.elements[13] += translation.y;
        self.elements[14] += translation.z;
    }

    pub fn scale(&mut self, scale: &Vector3) {
        self.elements[0] *= scale.x;
        self.elements[5] *= scale.y;
        self.elements[10] *= scale.z;
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let sin = f32::sin(angle);
        let cos = f32::cos(angle);
        let temp = self.elements[1];
        self.elements[1] = cos * temp - sin * self.elements[2];
        self.elements[2] = sin * temp + cos * self.elements[2];
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let sin = f32::sin(angle);
        let cos = f32::cos(angle);
        let temp = self.elements[0];
        self.elements[0] = cos * temp + sin * self.elements[2];
        self.elements[2] = -sin * temp + cos * self.elements[2];
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let sin = f32::sin(angle);
        let cos = f32::cos(angle);
        let temp = self.elements[0];
        self.elements[0] = cos * temp - sin * self.elements[1];
        self.elements[1] = sin * temp + cos * self.elements[1];
    }
}

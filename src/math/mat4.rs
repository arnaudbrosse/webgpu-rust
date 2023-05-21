use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Mat4 {
    elements: [[f32; 4]; 4],
}

impl Mat4 {
    pub(crate) fn new(elements: [[f32; 4]; 4]) -> Mat4 {
        Mat4 { elements }
    }
}

impl Add for Mat4 {
    type Output = Mat4;

    fn add(self, other: Mat4) -> Mat4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.elements[i][j] + other.elements[i][j];
            }
        }
        Mat4::new(result)
    }
}

impl Sub for Mat4 {
    type Output = Mat4;

    fn sub(self, other: Mat4) -> Mat4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.elements[i][j] - other.elements[i][j];
            }
        }
        Mat4::new(result)
    }
}

impl Mul<f32> for Mat4 {
    type Output = Mat4;

    fn mul(self, scalar: f32) -> Mat4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.elements[i][j] * scalar;
            }
        }
        Mat4::new(result)
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.elements[i][k] * other.elements[k][j];
                }
            }
        }
        Mat4::new(result)
    }
}

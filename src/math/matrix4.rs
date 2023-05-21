use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Matrix4 {
    elements: [[f32; 4]; 4],
}

impl Matrix4 {
    pub(crate) fn new(elements: [[f32; 4]; 4]) -> Matrix4 {
        Matrix4 { elements }
    }
}

impl Add for Matrix4 {
    type Output = Matrix4;

    fn add(self, other: Matrix4) -> Matrix4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.elements[i][j] + other.elements[i][j];
            }
        }
        Matrix4::new(result)
    }
}

impl Sub for Matrix4 {
    type Output = Matrix4;

    fn sub(self, other: Matrix4) -> Matrix4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.elements[i][j] - other.elements[i][j];
            }
        }
        Matrix4::new(result)
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, scalar: f32) -> Matrix4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.elements[i][j] * scalar;
            }
        }
        Matrix4::new(result)
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.elements[i][k] * other.elements[k][j];
                }
            }
        }
        Matrix4::new(result)
    }
}

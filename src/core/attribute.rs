pub struct Attribute {
    data: Vec<f32>,
    size: u32,
}

impl Attribute {
    pub fn new(data: Vec<f32>, size: u32) -> Self {
        Self { data, size }
    }
}

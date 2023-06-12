use crate::core::attribute::Attribute;
use std::collections::HashMap;
use wgpu::{Device, Encoder};

pub struct Geometry {
    pub attributes: HashMap<String, Attribute>,
}

impl Geometry {
    pub fn new(attributes: HashMap<String, Attribute>) -> Self {
        Self { attributes }
    }

    pub fn draw(device: Device, encoder: Encoder, material: Material) {
        // TODO: Implement
    }
}

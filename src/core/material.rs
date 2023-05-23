use crate::core::texture::Texture;
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use wgpu::Device;

enum Uniform {
    Number(f32),
    NumberArray(Vec<f32>),
    Float32Array(Vec<f32>),
    Texture(Texture),
}

pub struct Material {
    pub vertex: String,
    pub fragment: String,
    pub uniforms: HashMap<String, Uniform>,
}

impl Material {
    pub fn new(vertex: String, fragment: String, uniforms: HashMap<String, Uniform>) -> Self {
        Self {
            vertex,
            fragment,
            uniforms,
        }
    }
}

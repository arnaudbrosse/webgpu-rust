use crate::core::geometry::Geometry;
use crate::core::material::Material;

pub struct Mesh {
    geometry: Geometry,
    material: Material,
}

impl Mesh {
    pub fn new(geometry: Geometry, material: Material) -> Self {
        Self { geometry, material }
    }

    pub fn draw(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        self.geometry.draw(device, encoder, &self.material)
    }
}

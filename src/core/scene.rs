pub struct Scene {
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new(meshes: Vec<Mesh>) -> Self {
        Self { meshes }
    }

    pub fn draw(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        for mesh in &self.meshes {
            mesh.draw(device, encoder);
        }
    }
}
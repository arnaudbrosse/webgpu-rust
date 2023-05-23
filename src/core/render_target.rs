use crate::core::texture::Texture;
use wgpu::{
    Device, Extent3d, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage,
};

pub struct RenderTarget {
    texture: Texture,
    width: u32,
    height: u32,
}

impl RenderTarget {
    pub fn new(device: &Device, width: u32, height: u32) -> Self {
        let texture_descriptor = TextureDescriptor {
            label: Some("Render Target"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsage::RENDER_ATTACHMENT | TextureUsage::SAMPLED,
        };

        // Create the texture
        let texture = device.create_texture(&texture_descriptor);

        RenderTarget {
            texture,
            width,
            height,
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

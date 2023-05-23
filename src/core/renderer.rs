use wgpu::util::DeviceExt;
use wgpu::{Device, Queue, RenderPass};

pub struct Renderer {
    device: Device,
    queue: Queue,
    surface: wgpu::Surface,
    swap_chain: wgpu::SwapChain,
    pipeline: wgpu::RenderPipeline,
    size: winit::dpi::PhysicalSize<u32>,
}

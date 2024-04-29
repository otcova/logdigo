mod rect;

pub use rect::*;

pub struct Pipelines {
    rect: RectPipeline,
}

impl Pipelines {
    pub fn new(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            rect: RectPipeline::new(device, surface_config),
        }
    }
}

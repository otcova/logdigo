mod rect;

pub use rect::*;

use crate::BindGroupLayouts;

pub struct Pipelines {
    rect: RectPipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &BindGroupLayouts,
    ) -> Self {
        Self {
            rect: RectPipeline::new(device, surface_config, bind_group_layouts),
        }
    }
}

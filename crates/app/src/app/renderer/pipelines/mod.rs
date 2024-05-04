mod line;
mod rect;

pub use line::*;
pub use rect::*;

use crate::BindGroupLayouts;

pub struct Pipelines {
    rect: RectPipeline,
    line: LinePipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &BindGroupLayouts,
    ) -> Self {
        Self {
            rect: RectPipeline::new(device, surface_config, bind_group_layouts),
            line: LinePipeline::new(device, surface_config, bind_group_layouts),
        }
    }
}

mod line;
mod rect;
mod round_rect;

pub use line::*;
pub use rect::*;
pub use round_rect::*;

use crate::BindGroupLayouts;

pub struct Pipelines {
    round_rect: RoundRectPipeline,
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
            round_rect: RoundRectPipeline::new(device, surface_config, bind_group_layouts),
            rect: RectPipeline::new(device, surface_config, bind_group_layouts),
            line: LinePipeline::new(device, surface_config, bind_group_layouts),
        }
    }
}

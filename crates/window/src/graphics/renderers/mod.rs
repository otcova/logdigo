mod immediate;
mod retained;

use super::models::Camera;
use super::WgpuContext;

pub trait Renderer {
    fn prepare(&mut self, context: &WgpuContext);
    fn render<'a>(
        &'a mut self,
        pass: &mut wgpu::RenderPass<'a>,
        context: &WgpuContext,
        camera: &'a Camera,
    );
}

mod retained_buffer;

use super::Renderer;
use crate::graphics::util::GPUBuffer;
use crate::graphics::{models::*, WgpuContext};
use retained_buffer::*;

struct RetainedModel<M>
where
    M: Model,
    [(); M::Buffer::ARRAYS]: Sized,
{
    buffer: RetainedInstanceBuffer<M>,
    gpu_buffer: [GPUBuffer; M::Buffer::ARRAYS],
}

impl<M> RetainedModel<M>
where
    M: Model,
    [(); M::Buffer::ARRAYS]: Sized,
{
    pub fn add(&mut self, model: <M::Buffer as InstanceBuffer>::Instance) -> Handle {
        self.buffer.push(model)
    }
    pub fn remove(&mut self, handle: Handle) {
        self.buffer.remove(handle)
    }
}

impl<M> Renderer for RetainedModel<M>
where
    M: Model,
    [(); M::Buffer::ARRAYS]: Sized,
{
    fn prepare(&mut self, context: &WgpuContext) {
        /// Upload all the modified arrays to the gpu
        for (array_i, modified) in self.buffer.modified_bytes().into_iter().enumerate() {
            self.gpu_buffer[array_i].upload_range(context, modified.offset, modified.data);
        }
    }

    fn render<'a>(
        &'a mut self,
        pass: &mut wgpu::RenderPass<'a>,
        context: &WgpuContext,
        camera: &'a Camera,
    ) {
        // Get gpu buffer slices
        let gpu_slices = std::array::from_fn(|array_i| {
            let size = self.buffer.bytes_of(array_i as u32).len() as wgpu::BufferAddress;
            self.gpu_buffer[array_i].slice(..size)
        });

        M::render(pass, gpu_slices);
    }
}

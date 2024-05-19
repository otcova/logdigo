mod retained_buffer;

use crate::graphics::util::GPUBuffer;
use crate::graphics::{models::*, WgpuContext};
use retained_buffer::*;

pub struct RetainedModel<M>
where
    M: ModelPipeline,
    [(); M::Buffer::ARRAYS]: Sized,
{
    buffer: RetainedInstanceBuffer<M>,
    gpu_buffer: [GPUBuffer; M::Buffer::ARRAYS],
}

impl<M> RetainedModel<M>
where
    M: ModelPipeline,
    [(); M::Buffer::ARRAYS]: Sized,
{
    pub fn new(context: &WgpuContext) -> Self {
        Self {
            buffer: RetainedInstanceBuffer::new(),
            gpu_buffer: std::array::from_fn(|_| {
                GPUBuffer::new(context, wgpu::BufferUsages::VERTEX)
            }),
        }
    }
    pub fn add(&mut self, model: <M::Buffer as InstanceBuffer>::Instance) -> Id {
        self.buffer.push(model)
    }
    pub fn remove(&mut self, handle: Id) {
        self.buffer.remove(handle)
    }

    pub fn prepare(&mut self, context: &WgpuContext) {
        /// Upload all the modified arrays to the gpu
        for (array_i, modified) in self.buffer.modified_bytes().into_iter().enumerate() {
            self.gpu_buffer[array_i].upload_range(context, modified.offset, modified.data);
        }
    }

    pub fn render<'a>(
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

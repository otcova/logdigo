use std::ops::Range;

use crate::graphics::util::GPUBuffer;
use crate::graphics::{models::*, WgpuContext};

pub struct ImmediateModel<M>
where
    M: ModelPipeline,
    [(); M::Buffer::ARRAYS]: Sized,
{
    buffer: M::Buffer,
    buffer_ranges: [Range<u32>; M::Buffer::ARRAYS],
    gpu_buffer: GPUBuffer,
}

impl<M> ImmediateModel<M>
where
    M: ModelPipeline,
    [(); M::Buffer::ARRAYS]: Sized,
{
    pub fn new(context: &WgpuContext) -> Self {
        Self {
            buffer: Default::default(),
            buffer_ranges: std::array::from_fn(|_| 0..0),
            gpu_buffer: GPUBuffer::new(context, wgpu::BufferUsages::VERTEX),
        }
    }
    pub fn add(&mut self, instance: <M::Buffer as InstanceBuffer>::Instance) {
        self.buffer.push(instance);
    }

    pub fn prepare(&mut self, context: &WgpuContext) {
        self.gpu_buffer.upload_slices(context, self.buffer.bytes());
    }

    pub fn render<'a>(
        &'a mut self,
        pass: &mut wgpu::RenderPass<'a>,
        context: &WgpuContext,
        camera: &'a Camera,
    ) {
        // Get gpu buffer slices
        let mut buffer_offset = 0;
        let gpu_slices = std::array::from_fn(|array_i| {
            let slice_size = self.buffer.bytes_of(array_i as u32).len() as wgpu::BufferAddress;
            let range = buffer_offset..slice_size;
            buffer_offset += slice_size;

            self.gpu_buffer.slice(range)
        });

        M::render(pass, gpu_slices);

        self.buffer.clear();
    }
}

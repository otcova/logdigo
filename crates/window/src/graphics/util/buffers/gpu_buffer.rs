use crate::graphics::WgpuContext;
use std::ops::RangeBounds;

pub struct GPUBuffer {
    buffer: wgpu::Buffer,
    real_size: wgpu::BufferAddress,
}

impl GPUBuffer {
    pub fn new(context: &WgpuContext, usage: wgpu::BufferUsages) -> Self {
        let buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            size: 1024, // Minimum buffer size to prevent a lot of small buffer reallocations
            label: None,
            usage: wgpu::BufferUsages::COPY_DST | usage,
            mapped_at_creation: false,
        });
        Self {
            buffer,
            real_size: 0,
        }
    }

    /// Precondition: The range must be inside the buffer
    pub fn upload_range(
        &mut self,
        context: &WgpuContext,
        offset: wgpu::BufferAddress,
        data: &[u8],
    ) {
        // TODO: Check performance with Option 1 and 2 (https://github.com/gfx-rs/wgpu/discussions/1438)

        // Option 1: using write_buffer (hypothesis: is better on web)
        context.queue.write_buffer(&self.buffer, offset, data);

        // // Option 2: mapping the buffer (hypothesis: is equal or worse)
        // let end = offset + data.len() as wgpu::BufferAddress;
        // let mut view = self.buffer.slice(offset..end).get_mapped_range_mut();
        // view.copy_from_slice(data);
    }
    pub fn upload_slices<const N: usize>(&mut self, context: &WgpuContext, slices: [&[u8]; N]) {
        let total_bytes: wgpu::BufferAddress = slices.iter().map(|d| d.len() as u64).sum();
        self.grow(context, total_bytes);
        self.real_size = total_bytes;

        // TODO: Check performance with Option 1 and 2 (https://github.com/gfx-rs/wgpu/discussions/1438)

        // // Option 1: using write_buffer (hypothesis: better on web)
        // let mut offset = 0;
        // for data in slices {
        //     context.queue.write_buffer(&self.buffer, offset, data);
        //     offset += data.len() as wgpu::BufferAddress;
        // }

        // Option 2: mapping the buffer (hypothesis: better on desktop)
        let mut view = self.buffer.slice(..total_bytes).get_mapped_range_mut();
        let mut offset = 0;
        for data in slices {
            view[offset..data.len()].copy_from_slice(data);
            offset += data.len();
        }
    }

    pub fn slice<R: RangeBounds<wgpu::BufferAddress>>(&self, range: R) -> wgpu::BufferSlice {
        self.buffer.slice(range)
    }

    /// If the current gpu buffer is smaller than `min_size`,
    /// it will reallocate it so it has a capacity of at least `min_size`
    pub fn grow(&mut self, context: &WgpuContext, min_size: wgpu::BufferAddress) {
        if self.buffer.size() < min_size {
            self.buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
                mapped_at_creation: false,
                usage: self.buffer.usage(),
                label: None,
                size: min_size,
            });
        }
    }
}

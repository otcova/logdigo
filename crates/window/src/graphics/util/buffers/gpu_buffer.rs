use crate::graphics::WgpuContext;
use std::ops::RangeBounds;

pub struct GPUBuffer {
    buffer: wgpu::Buffer,
}

impl GPUBuffer {
    pub fn new(context: &WgpuContext, usage: wgpu::BufferUsages) -> Self {
        let buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            size: 256, // Minimum buffer size to prevent a lot of small buffer reallocations
            label: None,
            usage: wgpu::BufferUsages::COPY_DST | usage,
            mapped_at_creation: false,
        });
        Self { buffer }
    }

    pub fn upload_range(
        &mut self,
        context: &WgpuContext,
        cmd: &mut wgpu::CommandEncoder,
        offset: wgpu::BufferAddress,
        data: &[u8],
    ) {
        if data.len() > 0 {
            let min_size = offset + data.len() as wgpu::BufferAddress;

            let mut grow_slice = self.grow(context, Some(cmd), min_size, offset);
            if let Some(ref mut mapped) = grow_slice {
                mapped.copy_from_slice(data);
                drop(grow_slice); // Lifetime hell
                self.buffer.unmap();
            } else {
                drop(grow_slice); // Lifetime hell
                context.queue.write_buffer(&self.buffer, offset, data);
            }
        }
    }

    /// Optimized multi squential range uploads
    pub fn upload_slices<const N: usize>(&mut self, context: &WgpuContext, slices: [&[u8]; N]) {
        let total_bytes: wgpu::BufferAddress = slices.iter().map(|d| d.len() as u64).sum();

        let mut grow_slice = self.grow(context, None, total_bytes, 0);
        if let Some(ref mut mapped) = grow_slice {
            let mut offset = 0;
            for data in slices {
                mapped[offset..data.len()].copy_from_slice(data);
                offset += data.len();
            }
            drop(grow_slice); // Lifetime hell
            self.buffer.unmap();
        } else {
            drop(grow_slice); // Lifetime hell
            let mut offset = 0;
            for data in slices {
                context.queue.write_buffer(&self.buffer, offset, data);
                offset += data.len() as wgpu::BufferAddress;
            }
        }
    }

    pub fn slice<R: RangeBounds<wgpu::BufferAddress>>(&self, range: R) -> wgpu::BufferSlice {
        self.buffer.slice(range)
    }

    /// If the current gpu buffer is smaller than `min_size`,
    /// it will reallocate it so it has a capacity of at least `min_size`.
    ///
    /// If `min_size` already fits in the gpu buffer, the function returns immediatly.
    ///
    /// `conserve_bytes` the amount of bytes that are copied from the smaller buffer to the new
    /// larger buffer.
    ///
    /// # Return
    /// It returns the mapped_at_creation slice. Once written, it should be `buffer.unmap()`.
    fn grow(
        &mut self,
        context: &WgpuContext,
        cmd: Option<&mut wgpu::CommandEncoder>,
        min_size: wgpu::BufferAddress,
        conserve_bytes: wgpu::BufferAddress,
    ) -> Option<wgpu::BufferViewMut> {
        if self.buffer.size() < min_size {
            let new_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
                mapped_at_creation: true,
                usage: self.buffer.usage(),
                label: None,
                size: Self::align_buffer_size(min_size),
            });

            if let Some(cmd) = cmd {
                if conserve_bytes > 0 {
                    let copy_size = Self::align_buffer_size(conserve_bytes);
                    cmd.copy_buffer_to_buffer(&self.buffer, 0, &new_buffer, 0, copy_size);
                }
            }

            self.buffer = new_buffer;
            return Some(
                self.buffer
                    .slice(conserve_bytes..min_size)
                    .get_mapped_range_mut(),
            );
        }
        None
    }

    /// Valid vulkan usage is
    /// 1. buffer size must be a multiple of COPY_BUFFER_ALIGNMENT.
    /// 2. buffer size must be greater than 0.
    /// Therefore we round the value up to the nearest multiple, and ensure it's at least COPY_BUFFER_ALIGNMENT.
    fn align_buffer_size(unpadded_size: wgpu::BufferAddress) -> wgpu::BufferAddress {
        let align_mask = wgpu::COPY_BUFFER_ALIGNMENT - 1;
        ((unpadded_size + align_mask) & !align_mask).max(wgpu::COPY_BUFFER_ALIGNMENT)
    }
}

use super::GPUBuffer;
use crate::graphics::util::IdVec;
use crate::graphics::WgpuContext;
use bytemuck::NoUninit;
use std::ops::Range;

pub struct GPUIdVec<T: NoUninit> {
    data: IdVec<T>,
    buffer: GPUBuffer,
    modified_range: Range<u32>,
}

impl<T: NoUninit> GPUIdVec<T> {
    pub fn new(context: &WgpuContext, usage: wgpu::BufferUsages) -> Self {
        Self {
            data: IdVec::default(),
            buffer: GPUBuffer::new(context, usage),
            modified_range: 0..0,
        }
    }

    pub fn add(&mut self, item: T) -> u32 {
        let index = self.data.add(item);
        self.set_modified(index..index + 1);
        index
    }

    pub fn remove(&mut self, index: u32) {
        self.data.remove(index);
    }

    pub fn upload(&mut self, context: &WgpuContext, cmd: &mut wgpu::CommandEncoder) {
        let offset = self.modified_range.start as wgpu::BufferAddress;

        let data = &self.data.as_slice();
        let start = self.modified_range.start as usize;
        let end = usize::min(data.len(), self.modified_range.end as usize);
        let bytes = bytemuck::cast_slice(&data[start..end]);

        self.buffer.upload_range(context, cmd, offset, bytes);
    }

    fn set_modified(&mut self, range: Range<u32>) {
        if self.modified_range.is_empty() {
            self.modified_range = range;
        } else {
            let start = u32::min(range.start, self.modified_range.start);
            let end = u32::min(range.end, self.modified_range.end);
            self.modified_range = start..end;
        }
    }
}

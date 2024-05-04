use bytemuck::NoUninit;

use crate::*;
use std::{mem::size_of, ops::Range};

pub type InstanceId = u32;

pub struct InstanceBuffer<T: NoUninit> {
    instances: IdVec<T>,
    pub gpu_buffer: wgpu::Buffer,
}

impl<T: NoUninit> InstanceBuffer<T> {
    pub fn new(renderer: &Renderer) -> Self {
        let gpu_buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("LinesBatch Buffer"),
            size: 256, // TODO: Do not hardcode the initial size
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self {
            instances: IdVec::new(),
            gpu_buffer,
        }
    }

    pub fn update_buffers(&mut self, encoder: &mut RendererEncoder, renderer: &mut Renderer) {
        let updated_range = self.instances.reset_updated_range();
        let updated_instances = &self.instances.as_slice()[updated_range.clone()];
        let updated_bytes: &[u8] = bytemuck::cast_slice(updated_instances);

        if let Some(size) = std::num::NonZeroU64::new(updated_bytes.len() as u64) {
            if self.gpu_buffer.size() < size.into() {
                todo!("Resize the buffer");
            }
            let offset = (size_of::<T>() * updated_range.start) as wgpu::BufferAddress;
            let mut buffer_view = renderer.staging_belt.write_buffer(
                &mut *encoder,
                &self.gpu_buffer,
                offset,
                size,
                &renderer.device,
            );
            buffer_view.copy_from_slice(updated_bytes);
        }
    }

    pub fn remove(&mut self, id: InstanceId) {
        self.instances.remove(id)
    }

    pub fn push(&mut self, instance: T) -> InstanceId {
        self.instances.push(instance)
    }

    pub fn get_mut(&mut self, id: InstanceId) -> &mut T {
        self.instances.get_mut(id).unwrap()
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }
}

use bytemuck::NoUninit;
use wgpu::{util::DeviceExt, BufferAddress};

use crate::*;
use std::{
    mem::size_of,
    num::NonZeroU64,
    ops::{Range, RangeBounds},
};

pub type InstanceId = u32;

pub struct InstanceBuffer<T: NoUninit> {
    instances: IdVec<T>,
    instances_buffer: wgpu::Buffer,
    draw_buffer: wgpu::Buffer,
    past_instance_count: u32,
}

impl<T: NoUninit> InstanceBuffer<T> {
    pub fn new(renderer: &Renderer) -> Self {
        let instances_buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("LinesBatch Instances Buffer"),
            size: 256, // TODO: Do not hardcode the initial size
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let past_instance_count = 0;
        let indirect_args = wgpu::util::DrawIndirectArgs {
            vertex_count: 4,
            instance_count: past_instance_count,
            first_vertex: 0,
            first_instance: 0,
        };
        let draw_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("LinesBatch DrawArgs Buffer"),
                contents: indirect_args.as_bytes(),
                usage: wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::COPY_DST,
            });
        Self {
            instances: IdVec::new(),
            instances_buffer,
            draw_buffer,
            past_instance_count,
        }
    }

    fn update_instance_buffer(&mut self, renderer: &mut Renderer) {
        let updated_range = self.instances.reset_updated_range();
        let updated_instances = &self.instances.as_slice()[updated_range.clone()];
        let updated_bytes: &[u8] = bytemuck::cast_slice(updated_instances);

        if let Some(size) = std::num::NonZeroU64::new(updated_bytes.len() as u64) {
            if self.instances_buffer.size() < size.into() {
                todo!("Resize the buffer");
            }
            let offset = (size_of::<T>() * updated_range.start) as wgpu::BufferAddress;
            let mut buffer_view = renderer.staging_belt.write_buffer(
                &mut renderer.commands,
                &self.instances_buffer,
                offset,
                size,
                &renderer.device,
            );
            buffer_view.copy_from_slice(updated_bytes);
        }
    }

    fn update_draw_buffer(&mut self, renderer: &mut Renderer) {
        let instance_count = self.instances.len() as u32;
        if instance_count != self.past_instance_count {
            self.past_instance_count = instance_count;
            let mut buffer_view = renderer.staging_belt.write_buffer(
                &mut renderer.commands,
                &self.draw_buffer,
                size_of::<u32>() as BufferAddress, // See struct `wgpu::util::DrawIndirectArgs`
                NonZeroU64::new(size_of::<u32>() as u64).unwrap(),
                &renderer.device,
            );
            buffer_view.copy_from_slice(&instance_count.to_le_bytes());
        }
    }

    pub fn update_buffers(&mut self, renderer: &mut Renderer) {
        self.update_draw_buffer(renderer);
        self.update_instance_buffer(renderer);
    }

    pub fn bundle_render<'a>(&'a self, bundle: &mut RenderBundleEncoder<'a>) {
        let len = 1;
        let bytes = size_of::<RectInstance>() * len;
        let instances_buffer = self.instances_buffer.slice(0..bytes as u64);
        bundle.set_vertex_buffer(0, instances_buffer);
        bundle.draw_indirect(&self.draw_buffer, 0);
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
}

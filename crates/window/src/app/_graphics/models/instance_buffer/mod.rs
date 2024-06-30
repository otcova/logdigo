mod id_vec;

use crate::*;
use bytemuck::NoUninit;
use id_vec::*;
use std::marker::PhantomData;
use std::mem::size_of;
use std::num::NonZeroU64;
use std::ops::{Range, RangeBounds};
use wgpu::{util::DeviceExt, BufferAddress};

use self::app::graphics::graphics_app::WgpuContext;

#[derive(Debug)]
pub struct InstanceId<T> {
    id: Id,
    marker: PhantomData<T>,
}

impl<T> InstanceId<T> {
    fn from(id: Id) -> Self {
        Self {
            id,
            marker: Default::default(),
        }
    }
}

pub struct InstanceBuffer<T: NoUninit> {
    instances: IdVec<T>,
    instances_buffer: wgpu::Buffer,
    draw_buffer: wgpu::Buffer,
    past_instance_count: u32,
}

pub enum BufferUpdateStatus {
    Done,
    NeedsRebundle,
}

impl BufferUpdateStatus {
    pub fn all_done(status: &[Self]) -> bool {
        for s in status {
            if !matches!(s, Self::Done) {
                return false;
            }
        }
        true
    }
}

impl<T: NoUninit> InstanceBuffer<T> {
    pub fn new(ctx: &WgpuContext) -> Self {
        let instances_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: 0,
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
        let draw_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer DrawArgs"),
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

    fn update_instance_buffer(&mut self, ctx: &mut WgpuContext) -> BufferUpdateStatus {
        let updated_range = self.instances.reset_updated_range();
        let updated_instances = &self.instances.as_slice()[updated_range.clone()];
        let updated_bytes: &[u8] = bytemuck::cast_slice(updated_instances);

        if let Some(size) = std::num::NonZeroU64::new(updated_bytes.len() as u64) {
            if self.instances_buffer.size() < size.into() {
                self.instances_buffer =
                    ctx.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Instance Buffer"),
                            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                            contents: updated_bytes,
                        });
                return BufferUpdateStatus::NeedsRebundle;
            }

            let offset = (size_of::<T>() * updated_range.start) as wgpu::BufferAddress;
            let mut buffer_view = ctx.staging_belt.write_buffer(
                &mut ctx.commands,
                &self.instances_buffer,
                offset,
                size,
                &ctx.device,
            );
            buffer_view.copy_from_slice(updated_bytes);
        }

        BufferUpdateStatus::Done
    }

    fn update_draw_buffer(&mut self, ctx: &mut WgpuContext) {
        let instance_count = self.instances.len() as u32;
        if instance_count != self.past_instance_count {
            self.past_instance_count = instance_count;
            let mut buffer_view = ctx.staging_belt.write_buffer(
                &mut ctx.commands,
                &self.draw_buffer,
                size_of::<u32>() as BufferAddress, // See struct `wgpu::util::DrawIndirectArgs`
                NonZeroU64::new(size_of::<u32>() as u64).unwrap(),
                &ctx.device,
            );
            buffer_view.copy_from_slice(&instance_count.to_le_bytes());
        }
    }

    pub fn update_buffers(&mut self, ctx: &mut WgpuContext) -> BufferUpdateStatus {
        self.update_draw_buffer(ctx);
        self.update_instance_buffer(ctx)
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        let len = 1;
        let bytes = size_of::<T>() * len;
        let instances_buffer = self.instances_buffer.slice(0..bytes as u64);
        pass.set_vertex_buffer(0, instances_buffer);
        pass.draw_indirect(&self.draw_buffer, 0);
    }

    pub fn bundle_render<'a>(&'a self, bundle: &mut wgpu::RenderBundleEncoder<'a>) {
        let len = 1;
        let bytes = size_of::<T>() * len;
        let instances_buffer = self.instances_buffer.slice(0..bytes as u64);
        bundle.set_vertex_buffer(0, instances_buffer);
        bundle.draw_indirect(&self.draw_buffer, 0);
    }

    pub fn remove(&mut self, id: InstanceId<T>) {
        self.instances.remove(id.id)
    }

    pub fn push(&mut self, instance: T) -> InstanceId<T> {
        InstanceId::from(self.instances.push(instance))
    }

    pub fn get_mut(&mut self, id: InstanceId<T>) -> &mut T {
        self.instances.get_mut(id.id).unwrap()
    }
}

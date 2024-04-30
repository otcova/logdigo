use crate::*;
use bytemuck::{Pod, Zeroable};
use std::{mem::size_of, num::NonZero};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Camera2dUniform {
    min_corner: Vec2<f32>,
    max_corner: Vec2<f32>,
}

pub struct Camera2d {
    buffer: wgpu::Buffer,
    uniform: Camera2dUniform,
    bind_group: wgpu::BindGroup,
    changed: bool,
}

impl Camera2d {
    pub fn new(renderer: &Renderer) -> Self {
        let uniform = Camera2dUniform {
            min_corner: Vec2::splat(-10.0),
            max_corner: Vec2::splat(10.),
        };
        let buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size_of::<Camera2dUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &renderer.bind_group_layouts.camera,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some("Camera2d"),
            });

        Self {
            uniform,
            buffer,
            bind_group,
            changed: true,
        }
    }

    pub fn update_buffer(&mut self, encoder: &mut RendererEncoder, renderer: &mut Renderer) {
        if self.changed {
            let size = Camera2dUniform::SIZE;
            let encoder = &mut encoder.encoder;
            let belt = &mut renderer.staging_belt;
            let mut view = belt.write_buffer(encoder, &self.buffer, 0, size, &renderer.device);

            view.copy_from_slice(bytemuck::bytes_of(&self.uniform));
            self.changed = false;
        }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
    }
}

impl Camera2dUniform {
    /// Size in bytes of `Camera2dUniform`
    pub const SIZE: wgpu::BufferSize =
        // SAFETY: Camera2dUniform has size != 0
        unsafe { NonZero::new_unchecked(size_of::<Camera2dUniform>() as u64) };

    pub const fn layout() -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(Self::SIZE),
            },
            count: None,
        }
    }
}

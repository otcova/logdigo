use bytemuck::{Pod, Zeroable};
use derive_more::Deref;
use digolog_math::Vec2;
use std::mem::size_of;
use std::num::NonZero;
use std::ops::DerefMut;

use crate::graphics::WgpuContext;

#[derive(Deref)]
pub struct Camera {
    #[deref]
    uniform: CameraUniform,
    gpu_buffer: wgpu::Buffer,
    /// This will be true when the `uniform` data is borrowed mutably.
    /// And will be set false once the `uniform` is updloaded to the `gpu_buffer`.
    needs_upload: bool,
}

impl Camera {
    pub fn new(context: &WgpuContext) -> Self {
        Self {
            gpu_buffer: context.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Camera"),
                size: size_of::<CameraUniform>() as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                mapped_at_creation: false,
            }),
            uniform: CameraUniform {
                center: Vec2::splat(0.),
                clipping_scale: Vec2::splat(1.),
                pixel_scale: 1.,
            },
            needs_upload: true,
        }
    }

    pub fn prepare(&mut self, context: &WgpuContext) {
        if self.needs_upload {
            self.needs_upload = false;
            let bytes = bytemuck::bytes_of(&self.uniform);
            context.queue.write_buffer(&self.gpu_buffer, 0, bytes);
        }
    }
}

impl DerefMut for Camera {
    fn deref_mut(&mut self) -> &mut CameraUniform {
        self.needs_upload = true;
        &mut self.uniform
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraUniform {
    /// In word units
    pub center: Vec2<f32>,
    /// The amount of clipping units in a world unit
    clipping_scale: Vec2<f32>,
    /// The amount of pixels in a world unit
    pub pixel_scale: f32,
}

impl CameraUniform {
    /// Size in bytes of `Camera2dUniform`
    const SIZE: wgpu::BufferSize =
        // SAFETY: Camera2dUniform has size != 0
        unsafe { NonZero::new_unchecked(size_of::<Self>() as u64) };
}

pub(super) fn create_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("ImageLayout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(CameraUniform::SIZE),
            },
            count: None,
        }],
    })
}

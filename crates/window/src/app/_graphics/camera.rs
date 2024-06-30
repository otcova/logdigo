use super::WgpuContext;
use bytemuck::*;
use std::mem::size_of;
use std::num::NonZero;

pub(super) struct CameraModel {
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl CameraModel {
    pub const BIND_GROUP_LAYOUT: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: Some(Camera2dUniform::SIZE),
        },
        count: None,
    };

    pub fn new(ctx: &WgpuContext) -> Self {
        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[Self::BIND_GROUP_LAYOUT],
                    label: Some("Camera"),
                });
        Self { bind_group_layout }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Camera2dUniform {
    /// In word units
    center: [f32; 2],
    /// The amount of clipping units in a world unit
    clipping_scale: [f32; 2],
    /// The amount of pixels in a world unit
    pixel_scale: f32,
}

impl Camera2dUniform {
    /// Size in bytes of `Camera2dUniform`
    const SIZE: wgpu::BufferSize =
        // SAFETY: Camera2dUniform has size != 0
        unsafe { NonZero::new_unchecked(size_of::<Camera2dUniform>() as u64) };
}

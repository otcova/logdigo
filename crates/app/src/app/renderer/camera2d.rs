use crate::*;
use bytemuck::{Pod, Zeroable};
use derive_more::Deref;
use std::{mem::size_of, num::*, ops::DerefMut};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Camera2dUniform {
    /// In word units
    center: f32x2,
    /// The amount of clipping units in a world unit
    scale: f32x2,
}

#[derive(Copy, Clone, Debug)]
pub struct Camera2d {
    /// In word units
    center: f32x2,
    /// The amount of pixels in a world unit
    scale: f32,
}

#[derive(Deref)]
pub struct Camera2dBuffer {
    buffer: wgpu::Buffer,
    surface_pixels: u32x2,
    pub bind_group: wgpu::BindGroup,
    changed: bool,
    #[deref]
    camera: Camera2d,
}

impl Camera2dBuffer {
    pub fn new(renderer: &Renderer) -> Self {
        let camera = Camera2d {
            center: f32x2::splat(0.0),
            scale: 10.,
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
            buffer,
            bind_group,
            surface_pixels: u32x2::default(),
            camera,
            changed: true,
        }
    }

    pub fn resize(&mut self, new_size: u32x2) {
        if (self.surface_pixels != new_size) {
            self.surface_pixels = new_size;
            self.changed = true;
        }
    }

    pub fn update_buffer(&mut self, encoder: &mut RendererEncoder, renderer: &mut Renderer) {
        if self.changed {
            self.changed = false;

            let size = Camera2dUniform::SIZE;
            let encoder = &mut encoder.encoder;
            let belt = &mut renderer.staging_belt;
            let mut view = belt.write_buffer(encoder, &self.buffer, 0, size, &renderer.device);

            let uniform = Camera2dUniform::from_camera(**self, self.surface_pixels.cast());
            view.copy_from_slice(bytemuck::bytes_of(&uniform));
        }
    }
}

impl DerefMut for Camera2dBuffer {
    fn deref_mut(&mut self) -> &mut Camera2d {
        self.changed = true;
        &mut self.camera
    }
}

impl Camera2dUniform {
    pub fn from_camera(camera: Camera2d, surface_pixels: f32x2) -> Self {
        Self {
            center: camera.center,
            scale: f32x2::splat(camera.scale * 2.0) / surface_pixels,
        }
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

use crate::*;

pub struct BindGroupLayouts {
    pub camera: wgpu::BindGroupLayout,
    pub render_texture: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &wgpu::Device) -> Self {
        let camera = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[Camera2dUniform::BIND_GROUP_LAYOUT],
            label: Some("Camera"),
        });
        let render_texture = device.create_bind_group_layout(&RenderTexture::BIND_GROUP_LAYOUT);
        Self {
            camera,
            render_texture,
        }
    }
}

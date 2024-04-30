use crate::*;

pub struct BindGroupLayouts {
    pub camera: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &wgpu::Device) -> Self {
        let camera = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[Camera2dUniform::layout()],
            label: Some("Camera"),
        });
        Self { camera }
    }
}

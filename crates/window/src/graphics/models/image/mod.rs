use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use digolog_math::{Rect, Vec2, Vec3};

use crate::graphics::util::TextureRect;

use super::*;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ImageInstance {
    pub rect: Rect<f32>,
    pub texture: TextureRect,
    pub z_pos: u16,
}

pub struct ImagePipeline {
    pipeline: wgpu::RenderPipeline,
}

vec_instance_buffer!(
    struct ImageBuffer {
        instances: Vec<ImageInstance>,
    }
);

impl ImagePipeline {
    pub(super) fn create_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ImageLayout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }
}

impl ModelPipeline for ImagePipeline {
    type Buffer = ImageBuffer;

    fn new(context: &WgpuContext) -> Self {
        let device = &context.device;
        let shader = device.create_shader_module(wgpu::include_wgsl!("image.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &context.bind_group_layouts.camera,
                    &context.bind_group_layouts.image,
                ],
                push_constant_ranges: &[],
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("RectRenderer pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[ImageInstance::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: context.surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        ImagePipeline { pipeline }
    }

    fn render(&self, pass: &mut wgpu::RenderPass, slices: [wgpu::BufferSlice; 1]) {}
}

impl ImageInstance {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Unorm16x2, 3 => Unorm16x2, 4 => Uint32];

    const fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBUTES,
        }
    }
}

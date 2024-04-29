use std::mem::size_of;

use crate::*;
use bytemuck::{Pod, Zeroable};
use derive_more::Deref;
use wgpu::util::DeviceExt;

#[derive(Deref)]
pub struct RectPipeline {
    pipeline: wgpu::RenderPipeline,
}

pub struct RectsBatch {
    rects: Vec<ObjectId>,
    buffer_data: Vec<RectInstance>,
    buffer: wgpu::Buffer,
    rects_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct RectInstance {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl RectPipeline {
    pub fn new(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("rect.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("RectRenderer pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[RectInstance::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
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

        Self { pipeline }
    }
}

impl RectsBatch {
    const INSTANCES: &'static [RectInstance] = &[
        RectInstance {
            position: [0.0, 0.5, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        RectInstance {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        RectInstance {
            position: [0.5, -0.5, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ];

    pub fn new(renderer: &Renderer) -> Self {
        // let buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
        //     label: Some("RectsBatch Buffer"),
        //     size: 0,
        //     usage: wgpu::BufferUsages::VERTEX,
        //     mapped_at_sreation: false,
        // });
        let buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("RectsBatch Buffer"),
                contents: bytemuck::cast_slice(Self::INSTANCES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        Self {
            buffer,
            buffer_data: vec![],
            rects: vec![],
            rects_count: Self::INSTANCES.len() as u32,
        }
    }

    pub fn render<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, renderer: &'a Renderer) {
        //
        // TODO: Update buffer
        //

        render_pass.set_pipeline(&*renderer.pipelines.rect);
        let bytes = size_of::<RectInstance>() * self.rects_count as usize;
        render_pass.set_vertex_buffer(0, self.buffer.slice(0..bytes as wgpu::BufferAddress));
        render_pass.draw(0..4, 0..self.rects_count);
    }
}

impl RectInstance {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    const fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<RectInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

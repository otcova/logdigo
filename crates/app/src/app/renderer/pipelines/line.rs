use crate::*;
use bytemuck::{Pod, Zeroable};
use derive_more::Deref;
use std::{mem::size_of, ops::Range};
use wgpu::util::DeviceExt;

#[derive(Deref)]
pub struct LinePipeline {
    pipeline: wgpu::RenderPipeline,
}

pub struct LinesBatch {
    instances: InstanceBuffer<LineInstance>,
    render_bundle: Option<wgpu::RenderBundle>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LineInstance {
    pub position_a: [f32; 2],
    pub position_b: [f32; 2],
    pub color: u8x4,
}

impl LinePipeline {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &BindGroupLayouts,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("line.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layouts.camera],
                push_constant_ranges: &[],
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("LineRenderer pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[LineInstance::layout()],
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

impl LinesBatch {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            instances: InstanceBuffer::new(renderer),
            render_bundle: None,
        }
    }

    pub fn update_buffers(&mut self, renderer: &mut Renderer) {
        self.instances.update_buffers(renderer);
    }

    pub fn bundle_render<'a>(
        &'a self,
        bundle: &mut RenderBundleEncoder<'a>,
        renderer: &'a Renderer,
    ) {
        bundle.set_pipeline(&renderer.pipelines.line);
        self.instances.bundle_render(bundle);
    }

    pub fn push(&mut self, line: LineInstance) -> InstanceId {
        self.instances.push(line)
    }
}

impl LineInstance {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Unorm8x4];

    const fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<LineInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBUTES,
        }
    }
}

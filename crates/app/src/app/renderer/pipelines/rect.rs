use crate::*;
use bytemuck::{Pod, Zeroable};
use derive_more::Deref;
use std::{mem::size_of, ops::Range};
use wgpu::util::DeviceExt;

#[derive(Deref)]
pub struct RectPipeline {
    pipeline: wgpu::RenderPipeline,
}

pub struct RectsBatch {
    rects: Vec<ObjectId>,
    instances: Vec<RectInstance>,
    buffer: wgpu::Buffer,
    updated_range: Range<usize>,
    render_bundle: Option<wgpu::RenderBundle>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct RectInstance {
    pub position: f32x2,
    pub size: u16x2,
    pub color: u8x4,
}

impl RectPipeline {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &BindGroupLayouts,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("rect.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layouts.camera],
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
    pub fn new(renderer: &Renderer) -> Self {
        let buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("RectsBatch Buffer"),
            size: 256, // TODO: Do not hardcode the initial size
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self {
            buffer,
            instances: vec![],
            rects: vec![],
            updated_range: 0..0,
            render_bundle: None,
        }
    }

    pub fn update_buffers(&mut self, encoder: &mut RendererEncoder, renderer: &mut Renderer) {
        let updated_instances = &self.instances[self.updated_range.clone()];
        let data: &[u8] = bytemuck::cast_slice(updated_instances);
        let size = data.len() as u64;
        if let Some(size) = std::num::NonZeroU64::new(size) {
            let offset =
                (size_of::<RectInstance>() * self.updated_range.start) as wgpu::BufferAddress;
            let mut buffer_view = renderer.staging_belt.write_buffer(
                &mut encoder.encoder,
                &self.buffer,
                offset,
                size,
                &renderer.device,
            );
            buffer_view.copy_from_slice(data);
            self.updated_range = 0..0;
        }
    }

    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        renderer: &'a mut Renderer,
        camera: &'a Camera2d,
    ) {
        let render_bundle = self.render_bundle.get_or_insert_with(|| {
            let mut enc = renderer.device.create_render_bundle_encoder(
                &wgpu::RenderBundleEncoderDescriptor {
                    label: Some("Rect"),
                    multiview: None,
                    sample_count: 1,
                    color_formats: &[Some(renderer.surface_config.format)],
                    depth_stencil: None,
                },
            );
            enc.set_pipeline(&*renderer.pipelines.rect);
            enc.set_bind_group(0, &camera.bind_group, &[]);
            let bytes = size_of::<RectInstance>() * self.instances.len();
            enc.set_vertex_buffer(0, self.buffer.slice(0..bytes as u64));
            enc.draw(0..4, 0..self.instances.len() as u32);

            enc.finish(&wgpu::RenderBundleDescriptor {
                label: Some("Rect"),
            })
        });

        render_pass.execute_bundles([&*render_bundle]);
    }

    pub fn insert(&mut self, id: ObjectId, rect: RectInstance) -> usize {
        self.rects.push(id);
        self.instances.push(rect);

        let updated = self.instances.len() - 1..self.instances.len();

        if self.updated_range.is_empty() {
            self.updated_range.start = updated.start;
        }
        self.updated_range.end = updated.end;

        updated.start
    }
}

impl RectInstance {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x2, 1 => Uint16x2, 2 => Unorm8x4];

    const fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<RectInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBUTES,
        }
    }
}

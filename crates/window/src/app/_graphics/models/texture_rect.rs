use super::camera::CameraModel;
use super::instance_buffer::*;
use super::{Models, WgpuContext};
use bytemuck::*;

pub struct TextureRectBatch {
    instances: InstanceBuffer<TextureRectInstance>,
}

impl TextureRectBatch {
    pub fn update_buffers(&mut self, ctx: &mut WgpuContext) {
        self.instances.update_buffers(ctx);
    }

    pub fn render<'a>(&'a self, models: &'a Models, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&models.texture_rect.pipeline);
        self.instances.render(pass)
    }

    pub fn push(&mut self, instance: TextureRectInstance) -> InstanceId<TextureRectInstance> {
        self.instances.push(instance)
    }
}

pub(super) struct TextureRectModel {
    pipeline: wgpu::RenderPipeline,
}

impl TextureRectModel {
    pub const BIND_GROUP_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> =
        wgpu::BindGroupLayoutDescriptor {
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
            label: Some("TextureRect"),
        };

    pub fn new(ctx: &WgpuContext, camera_model: &CameraModel) -> Self {
        let shader = ctx
            .device
            .create_shader_module(wgpu::include_wgsl!("texture_rect.wgsl"));

        let bind_group_layout = ctx
            .device
            .create_bind_group_layout(&Self::BIND_GROUP_LAYOUT);

        let render_pipeline_layout =
            ctx.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&camera_model.bind_group_layout, &bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("TextureRect"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[TextureRectInstance::LAYOUT],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: ctx.surface_config.format,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TextureRectInstance {
    pos: [f32; 3],
    size: f32,
    uv_pos: [u16; 2],
    uv_size: [u16; 2],
}

impl TextureRectInstance {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Unorm16x2, 2 => Unorm16x2];

    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &Self::ATTRIBUTES,
    };
}

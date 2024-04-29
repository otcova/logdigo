mod pipelines;

use std::{iter, sync::Arc};
use wgpu::util::StagingBelt;
use winit::{dpi::PhysicalSize, window::Window};

pub use pipelines::*;

/// Responsible to interact with wgpu
/// It's a specialized wgpu abstraction layer
pub struct Renderer {
    device: wgpu::Device,
    surface_config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    queue: wgpu::Queue,
    pub window: Arc<Window>,
    pub pipelines: Pipelines,
    pub staging_belt: wgpu::util::StagingBelt,
}

pub struct RendererEncoder {
    encoder: wgpu::CommandEncoder,
    output: wgpu::SurfaceTexture,
}

pub struct RendererTexture<'a> {
    view: wgpu::TextureView,
    encoder: &'a mut wgpu::CommandEncoder,
}

pub type RenderPass<'a> = wgpu::RenderPass<'a>;

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_limits: wgpu::Limits::default(),
                    required_features: wgpu::Features::empty(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let pipelines = Pipelines::new(&device, &surface_config);

        let staging_belt = StagingBelt::new(1 << 10); // TODO: Check this constant

        Self {
            device,
            surface_config,
            surface,
            queue,
            window,
            pipelines,
            staging_belt,
        }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        PhysicalSize {
            width: self.surface_config.width,
            height: self.surface_config.height,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 && self.size() != new_size {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
            self.window.request_redraw();
        }
    }

    pub fn create_encoder(&mut self) -> RendererEncoder {
        RendererEncoder {
            output: self.surface.get_current_texture().unwrap(),
            encoder: self.device.create_command_encoder(&Default::default()),
        }
    }

    pub fn render(&mut self, encoder: RendererEncoder) {
        self.staging_belt.finish();
        self.queue.submit(iter::once(encoder.encoder.finish()));
        self.staging_belt.recall();

        self.window.pre_present_notify();
        encoder.output.present();
    }
}

impl RendererEncoder {
    pub fn surface_texture_target(&mut self) -> RendererTexture {
        let view = self
            .output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        RendererTexture {
            view,
            encoder: &mut self.encoder,
        }
    }
}

impl<'a> RendererTexture<'a> {
    pub fn render_pass(&mut self) -> wgpu::RenderPass {
        let render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Main Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass
    }
}

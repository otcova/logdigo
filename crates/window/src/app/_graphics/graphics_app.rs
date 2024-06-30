use super::models::Models;
use digolog_math::*;
use std::sync::Arc;
use wgpu::util::StagingBelt;
use winit::window::Window;

pub(super) struct WgpuContext {
    pub device: wgpu::Device,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub queue: wgpu::Queue,
    pub commands: wgpu::CommandEncoder,
    pub staging_belt: wgpu::util::StagingBelt,
}

impl WgpuContext {
    pub fn size(&self) -> Vec2<u32> {
        [self.surface_config.width, self.surface_config.height].into()
    }

    fn resize(&mut self, new_size: Vec2<u32>) -> bool {
        if new_size > [0, 0].into() && self.size() != new_size {
            self.surface_config.width = new_size[0];
            self.surface_config.height = new_size[1];
            self.surface.configure(&self.device, &self.surface_config);
            true
        } else {
            false
        }
    }

    fn render(&mut self, render: impl FnOnce(&mut wgpu::RenderPass)) -> wgpu::SurfaceTexture {
        let output_surface = self.surface.get_current_texture().unwrap();
        let output_texture = output_surface.texture.create_view(&Default::default());

        {
            let mut render_pass = self
                .commands
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Main Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &output_texture,
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

            render(&mut render_pass);
        }

        let new_commands = self.device.create_command_encoder(&Default::default());
        let commands = std::mem::replace(&mut self.commands, new_commands);

        self.staging_belt.finish();
        self.queue.submit([commands.finish()]);
        self.staging_belt.recall();

        output_surface
    }

    pub fn create_bundle<'a, F>(&'a self, add_render_steps: F) -> wgpu::RenderBundle
    where
        F: FnOnce(&mut RenderBundleEncoder<'a>),
    {
        let mut bundle =
            self.device
                .create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                    label: None,
                    multiview: None,
                    sample_count: 1,
                    color_formats: &[Some(self.surface_config.format)],
                    depth_stencil: None,
                });

        add_render_steps(&mut bundle);

        bundle.finish(&Default::default())
    }
}

/// Responsible to interact with wgpu
/// It's a specialized wgpu abstraction layer
pub struct GraphicsApp {
    pub window: Arc<Window>,

    context: WgpuContext,

    models: Models,
}

pub use wgpu::RenderBundleEncoder;

pub type RenderPass<'a> = wgpu::RenderPass<'a>;

impl GraphicsApp {
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

        let commands = device.create_command_encoder(&Default::default());
        let staging_belt = StagingBelt::new(1 << 10); // TODO: Check this constant

        let context = WgpuContext {
            device,
            surface_config,
            surface,
            queue,
            commands,
            staging_belt,
        };

        let models = Models::new(&context);

        Self {
            window,
            context,
            models,
        }
    }

    /// Returns true if the size has changed
    pub fn resize(&mut self, new_size: Vec2<u32>) -> bool {
        if (self.context.resize(new_size)) {
            self.window.request_redraw();
            true
        } else {
            false
        }
    }

    pub fn render(&mut self, render: impl FnOnce(&mut wgpu::RenderPass)) {
        let output_surface = self.context.render(render);
        self.window.pre_present_notify();
        output_surface.present();
    }
}

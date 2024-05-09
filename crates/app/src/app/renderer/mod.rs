mod bind_groups;
mod camera2d;
mod pipelines;
mod texture;
mod util;

use crate::*;
use derive_more::*;
use std::{iter, sync::Arc};
use wgpu::util::StagingBelt;
use winit::window::Window;

pub use bind_groups::*;
pub use camera2d::*;
pub use pipelines::*;
pub use texture::*;
pub use util::*;

/// Responsible to interact with wgpu
/// It's a specialized wgpu abstraction layer
pub struct Renderer {
    device: wgpu::Device,
    surface_config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    queue: wgpu::Queue,
    pub window: Arc<Window>,
    pub pipelines: Pipelines,
    pub bind_group_layouts: BindGroupLayouts,
    pub staging_belt: wgpu::util::StagingBelt,
    commands: wgpu::CommandEncoder,
}

pub struct RenderPhase {
    bundle: wgpu::RenderBundle,
}

pub use wgpu::RenderBundleEncoder;

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

        let bind_group_layouts = BindGroupLayouts::new(&device);
        let pipelines = Pipelines::new(&device, &surface_config, &bind_group_layouts);

        let staging_belt = StagingBelt::new(1 << 10); // TODO: Check this constant

        let commands = device.create_command_encoder(&Default::default());
        let output_surface = surface.get_current_texture().unwrap();

        Self {
            device,
            surface_config,
            surface,
            queue,
            window,
            pipelines,
            staging_belt,
            bind_group_layouts,
            commands,
        }
    }

    pub fn size(&self) -> u32x2 {
        [self.surface_config.width, self.surface_config.height].into()
    }

    /// Returns true if the size has changed
    pub fn resize(&mut self, new_size: u32x2) -> bool {
        if new_size > [0, 0].into() && self.size() != new_size {
            self.surface_config.width = new_size[0];
            self.surface_config.height = new_size[1];
            self.surface.configure(&self.device, &self.surface_config);
            self.window.request_redraw();

            true
        } else {
            false
        }
    }

    pub fn render<'a>(&mut self, phases: impl IntoIterator<Item = &'a RenderPhase>) {
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

            for phase in phases {
                render_pass.execute_bundles([&phase.bundle]);
            }
        }

        let new_commands = self.device.create_command_encoder(&Default::default());
        let commands = std::mem::replace(&mut self.commands, new_commands);

        self.staging_belt.finish();
        self.queue.submit([commands.finish()]);
        self.staging_belt.recall();

        self.window.pre_present_notify();
        output_surface.present();
    }

    pub fn create_phase<'a, F>(&'a self, add_render_steps: F) -> RenderPhase
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

        RenderPhase {
            bundle: bundle.finish(&Default::default()),
        }
    }
}

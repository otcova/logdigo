use self::models::BindGroupLayouts;
use super::*;
use derive_more::*;

#[derive(Deref)]
pub struct WgpuContext(Arc<InnerWgpuContext>);

pub struct InnerWgpuContext {
    pub window: Arc<Window>,
    pub device: wgpu::Device,
    pub surface_format: wgpu::TextureFormat,
    pub surface: wgpu::Surface<'static>,
    pub queue: wgpu::Queue,
    pub bind_group_layouts: BindGroupLayouts,
}

impl WgpuContext {
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
        let size = Vec2::new(size.width, size.height);
        let surface_config = InnerWgpuContext::create_surface_config(size, surface_format);
        surface.configure(&device, &surface_config);

        let bind_group_layouts = BindGroupLayouts::new(&device);

        Self(Arc::new(InnerWgpuContext {
            window,
            device,
            surface_format,
            surface,
            queue,
            bind_group_layouts,
        }))
    }
}

impl InnerWgpuContext {
    fn create_surface_config(
        size: Vec2<u32>,
        format: wgpu::TextureFormat,
    ) -> wgpu::SurfaceConfiguration {
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.x,
            height: size.y,
            present_mode: wgpu::PresentMode::Mailbox,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    pub fn resize_surface(&self, new_size: Vec2<u32>) {
        let config = Self::create_surface_config(new_size, self.surface_format);
        self.surface.configure(&self.device, &config);
        self.window.request_redraw();
    }

    pub fn render(&self, render: impl FnOnce(&mut wgpu::RenderPass)) {
        let output_surface = self.surface.get_current_texture().unwrap();
        let output_texture = output_surface.texture.create_view(&Default::default());

        let mut commands = self.device.create_command_encoder(&Default::default());

        {
            let mut render_pass = commands.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        self.queue.submit([commands.finish()]);

        self.window.pre_present_notify();
        output_surface.present();
    }
}

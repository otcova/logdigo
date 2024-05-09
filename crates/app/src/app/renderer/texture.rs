use crate::*;

pub struct RenderTexture {
    texture: wgpu::Texture,
    sampler: wgpu::Sampler,
    bind_group: wgpu::BindGroup,
}

impl RenderTexture {
    fn new(renderer: &Renderer) -> Self {
        let size = wgpu::Extent3d {
            width: 100,
            height: 100,
            depth_or_array_layers: 1,
        };
        let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("RenderTexture"),
            size,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            mip_level_count: 1,
            format: renderer.surface_config.format,
            dimension: wgpu::TextureDimension::D2,
            sample_count: 1,
        });
        let data_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(100 * 4),
            rows_per_image: Some(100),
        };

        let mut data = vec![0u8; 4 * 100 * 100];

        let mut c = 0u32;
        for color in &mut data {
            *color = (c / 256) as u8;
            c += 1;
        }

        renderer.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All, // TODO: What's this exactly?
            },
            &data,
            data_layout,
            size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = renderer.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &renderer.bind_group_layouts.render_texture,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            });

        Self {
            texture,
            sampler,
            bind_group,
        }
    }

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
            label: Some("RenderTexture"),
        };
}

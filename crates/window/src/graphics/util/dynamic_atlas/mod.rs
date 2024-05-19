mod atlas3d;

use self::atlas3d::Atlas3d;
use crate::graphics::WgpuContext;
use digolog_math::{Rect, Vec2};
use guillotiere::{size2, Allocation, AtlasAllocator};
use wgpu::Extent3d;

pub use self::atlas3d::{AtlasHandle, TextureRect};

pub struct DynamicAtlas {
    atlas: Atlas3d,
    texture: Option<wgpu::Texture>,
    texture_view: Option<wgpu::TextureView>,
    /// Indicates if the atlas has been resized and the texture needs to be realocated.
    size_changed: bool,
}

impl DynamicAtlas {
    pub fn new(context: &WgpuContext) -> Self {
        let max_dimension_size = context.device.limits().max_texture_dimension_3d;
        Self {
            atlas: Atlas3d::new(max_dimension_size.try_into().unwrap_or(u16::MAX)),
            texture: None,
            texture_view: None,
            size_changed: true,
        }
    }

    pub fn add(&mut self, size: Vec2<u16>) -> AtlasHandle {
        if let Some(handle) = self.atlas.add(size) {
            handle
        } else {
            self.atlas.grow(size);
            self.size_changed = true;
            self.add(size)
        }
    }

    pub fn remove(&mut self, handle: AtlasHandle) {
        self.atlas.remove(handle);
    }

    pub fn prepare(&mut self, context: &WgpuContext, cmd: &mut wgpu::CommandEncoder) {
        if self.size_changed {
            self.create_texture(context, cmd);
        };
    }

    fn create_texture(&mut self, context: &WgpuContext, cmd: &mut wgpu::CommandEncoder) {
        self.size_changed = false;

        let new_size = self.atlas.size();
        let new_size = wgpu::Extent3d {
            width: new_size.x as u32,
            height: new_size.y as u32,
            depth_or_array_layers: new_size.z as u32,
        };

        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("DynamicAtlas Texture"),
            dimension: wgpu::TextureDimension::D3,
            size: new_size,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            format: context.surface_format,
            view_formats: &[context.surface_format],
            mip_level_count: 1,
            sample_count: 1,
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("DynamicAtlas TextureView"),
            ..Default::default()
        });

        /// Copy smaller previous texture data into new the texture
        if let Some(old_texture) = &self.texture {
            let old = wgpu::ImageCopyTexture {
                texture: &old_texture,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
                mip_level: 0,
            };
            let new = wgpu::ImageCopyTexture {
                texture: &texture,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
                mip_level: 0,
            };
            cmd.copy_texture_to_texture(old, new, old_texture.size());
        }

        self.texture = Some(texture);
        self.texture_view = Some(texture_view);
    }

    /// # Panic
    /// `prepare()` must be called before `texture_view()`
    pub fn texture_view(&self) -> &wgpu::TextureView {
        let Some(texture_view) = &self.texture_view else {
            panic!("prepare must be called before texture_view");
        };
        texture_view
    }
}

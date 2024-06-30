mod atlas3d;

use self::atlas3d::{AllocId, Atlas3d};
use super::GPUIdVec;
use crate::graphics::WgpuContext;
use bytemuck::{Pod, Zeroable};
use digolog_math::*;
use guillotiere::{size2, Allocation, AtlasAllocator};
use wgpu::Extent3d;

pub use self::atlas3d::TextureRect;

pub struct DynamicAtlas {
    atlas: Atlas3d,
    rects: GPUIdVec<TextureRect>,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    /// Indicates if the atlas has been resized and the texture needs to be realocated.
    size_changed: bool,
}

pub struct AtlasRectId {
    rect_id: u32,
    atlas_id: AllocId,
}

impl DynamicAtlas {
    pub fn new(context: &WgpuContext) -> Self {
        let max_dimension_size = context.device.limits().max_texture_dimension_3d;
        let atlas = Atlas3d::new(max_dimension_size.try_into().unwrap_or(u16::MAX));
        let (texture, texture_view) = Self::create_texture(context, atlas.size());

        Self {
            atlas,
            rects: GPUIdVec::new(&context, wgpu::BufferUsages::STORAGE),
            texture,
            texture_view,
            size_changed: false,
        }
    }

    pub fn add(&mut self, size: Vec2<u16>) -> AtlasRectId {
        if let Some(handle) = self.atlas.add(size) {
            let rect_id = self.rects.add(handle.rect());
            AtlasRectId {
                rect_id,
                atlas_id: handle.id(),
            }
        } else {
            self.atlas.grow(size);
            self.size_changed = true;
            self.add(size)
        }
    }

    pub fn remove(&mut self, handle: AtlasRectId) {
        self.rects.remove(handle.rect_id);
        self.atlas.remove(handle.atlas_id);
    }

    pub fn prepare(&mut self, context: &WgpuContext, cmd: &mut wgpu::CommandEncoder) {
        if self.size_changed {
            self.grow_texture(context, cmd);
        };
        self.rects.upload(context, cmd);
    }

    fn create_texture(
        context: &WgpuContext,
        size: Vec3<u16>,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let size = wgpu::Extent3d {
            width: size.x as u32,
            height: size.y as u32,
            depth_or_array_layers: size.z as u32,
        };

        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("DynamicAtlas Texture"),
            dimension: wgpu::TextureDimension::D3,
            size,
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

        (texture, texture_view)
    }

    /// Creates a larger texture and copies the data from the old one.
    fn grow_texture(&mut self, context: &WgpuContext, cmd: &mut wgpu::CommandEncoder) {
        self.size_changed = false;

        let new_size = self.atlas.size();
        let (texture, texture_view) = Self::create_texture(context, new_size);

        /// Copy smaller previous texture data into new the texture
        let old = wgpu::ImageCopyTexture {
            texture: &self.texture,
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
        cmd.copy_texture_to_texture(old, new, self.texture.size());

        self.texture = texture;
        self.texture_view = texture_view;
    }

    pub fn texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}

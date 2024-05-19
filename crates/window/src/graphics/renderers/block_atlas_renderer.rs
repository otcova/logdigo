use crate::graphics::models::Camera;
use crate::graphics::util::{AtlasHandle, DynamicAtlas, TextureRect};
use crate::graphics::WgpuContext;
use digolog_math::Vec2;
use digolog_module_loader::{BlockShape, BlockShapeId};
use guillotiere::euclid::num::Ceil;
use wgpu::naga::FastHashMap;

pub struct BlockAtlasRenderer {
    atlas: DynamicAtlas,
}

impl BlockAtlasRenderer {
    pub fn new(context: &WgpuContext) -> Self {
        Self {
            atlas: DynamicAtlas::new(context),
        }
    }

    /// The shape will be renderer into the atlas texture with the resolution of camera pixel_scale
    pub fn add(&mut self, shape: &BlockShape, camera: &Camera) -> AtlasHandle {
        let size = shape.size() * camera.pixel_scale;
        let texture_size = Vec2::new(size.x.ceil() as u16, size.y.ceil() as u16);
        let handle = self.atlas.add(texture_size);
        handle
    }

    pub fn remove(&mut self, handle: AtlasHandle) {
        self.atlas.remove(handle)
    }

    pub fn prepare(&mut self, context: &WgpuContext, cmd: &mut wgpu::CommandEncoder) {
        self.atlas.prepare(context, cmd);
    }

    pub fn render(&mut self) {
        // todo!()
    }
}

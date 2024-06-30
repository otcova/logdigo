//! Each model renders to a single target
//! with a single batched draw call for instance.

mod camera;
mod instance_buffer;
mod texture_rect;

pub use texture_rect::*;

use self::camera::CameraModel;

use super::graphics_app::WgpuContext;

/// Global data of a model that depend on the wgpu context
/// but not from the actual model instances.
/// This could be: pipelines, layouts, bind_groups
pub struct Models {
    texture_rect: TextureRectModel,
}

impl Models {
    pub fn new(ctx: &WgpuContext) -> Self {
        let camera_model = CameraModel::new(ctx);
        Self {
            texture_rect: TextureRectModel::new(ctx, &camera_model),
        }
    }
}

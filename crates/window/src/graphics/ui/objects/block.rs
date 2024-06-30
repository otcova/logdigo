use super::*;
use crate::graphics::models::{ImageInstance, ImagePipeline};
use crate::graphics::renderers::{Id, RetainedModel};
use crate::graphics::WgpuContext;
use digolog_math::{Rect, Vec2};
use digolog_module_loader::BlockShapeId;
use wgpu::naga::FastHashMap;

pub(crate) struct BlockCollection {
    blocks: RetainedModel<ImagePipeline>,
    shapes: FastHashMap<BlockShapeId, Vec<Id>>,
}

impl UIObjectCollection for BlockCollection {
    type Object = Block;

    fn new(context: &WgpuContext) -> Self {
        Self {
            blocks: RetainedModel::new(context),
            shapes: FastHashMap::default(),
        }
    }

    /// After the camera pixel_scale changes, the block textures will be updated
    /// on the following frames
    fn update_camera(&mut self, camera: &Camera) {}

    fn add(&mut self, block: Block, renderers: &mut SharedRenderers) -> Handle<Block> {
        // only if needed
        let texture = renderers.block_atlas.add(block.shape);
        let id = self.blocks.add(ImageInstance {
            rect: Rect::from_size(
                Vec2::new(block.pos.x as f32, block.pos.y as f32),
                Vec2::new(block.size.x as f32, block.size.y as f32),
            ),
            texture,
            z_pos: 0,
        });
        Handle::from_id(*id)
    }
}

pub struct Block {
    pos: Vec2<i32>,
    size: Vec2<u16>,
    shape: BlockShape,
}

impl_object!(Block, BlockCollection, blocks);
impl_object!(Block, BlockCollection, blocks);

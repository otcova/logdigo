use digolog_math::{Rect, Vec2};

use super::*;
use crate::graphics::models::{ImageInstance, ImagePipeline};
use crate::graphics::renderers::RetainedModel;
use crate::graphics::WgpuContext;

pub(crate) struct BlockCollection {
    blocks: RetainedModel<ImagePipeline>,
}

impl UIObjectCollection for BlockCollection {
    type Object = Block;

    fn new(context: &WgpuContext) -> Self {
        Self {
            blocks: RetainedModel::new(context),
        }
    }

    fn add(&mut self, block: Block, renderers: &mut SharedRenderers) -> Handle<Block> {
        let texture = renderers.block_atlas.add(block.shape);
        self.blocks.add(ImageInstance {
            rect: Rect::from_size(
                Vec2::new(block.pos.x as f32, block.pos.y as f32),
                Vec2::new(block.size.x as f32, block.size.y as f32),
            ),
            texture,
            z_pos: 0,
        });
    }
}

pub struct Block {
    pos: Vec2<i32>,
    size: Vec2<u16>,
    shape: BlockShape,
}

impl_object!(Block, BlockCollection, blocks);

mod atlas;
mod instant_renderers;

use crate::app::graphics::graphics_app::WgpuContext;
use atlas::*;
use digolog_math::*;
use instant_renderers::*;
use std::any::Any;

struct BlockBatch {
    atlas: BlockAtlas,
    instant_renderers: BlockInstantRenderers,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct BlockShape {
    size: Vec2<u16>,
    title: String,
    color: Color,
}

impl BlockBatch {
    fn new() -> Self {
        // TODO: Dinamically grow the texture size
        let initial_size = Vec2::splat(1 << 11);
        Self {
            atlas: BlockAtlas::new(initial_size),
            instant_renderers: BlockInstantRenderers::new(),
        }
    }

    fn add_block(&mut self, shape: &BlockShape) -> TextureAtlasRect {
        if let Some(rect) = self.atlas.share_shape(shape) {
            return rect;
        }

        if let Some(rect) = self.atlas.allocate_shape(shape.clone()) {
            todo!("Add BlockShape to render");
            self.instant_renderers.draw_block();
            return rect;
        }

        todo!("Resize texture");
    }

    fn remove_block() {}

    /// Render new blocks to the texture
    fn render(&mut self, ctx: &WgpuContext) {
        self.instant_renderers.render(ctx);
    }
}

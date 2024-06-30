use self::ui::BlockBuilder;
use crate::*;
use digolog_math::*;
use std::collections::HashMap;

pub struct BlockPainter {
    indexes: HashMap<ObjectId, BlockObject>,
    rects: RoundRectsBatch,
}

struct BlockObject {
    rect: InstanceId,
}

impl BlockPainter {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            indexes: HashMap::new(),
            rects: RoundRectsBatch::new(renderer),
        }
    }

    pub fn update_buffers(&mut self, renderer: &mut Renderer) -> BufferUpdateStatus {
        self.rects.update_buffers(renderer)
    }

    pub fn bundle_render<'a>(
        &'a self,
        bundle: &mut RenderBundleEncoder<'a>,
        renderer: &'a Renderer,
    ) {
        self.rects.bundle_render(bundle, renderer);
    }

    pub fn insert(&mut self, id: ObjectId, block: BlockBuilder) {
        let rect = self.rects.push(RoundRectInstance {
            position: block.position.cast(),
            size: block.size,
            color: *block.color,
        });

        self.indexes.insert(id, BlockObject { rect });
    }
}

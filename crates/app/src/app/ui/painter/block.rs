use crate::*;
use std::collections::HashMap;

use self::ui::BlockBuilder;

pub struct BlockPainter {
    indexes: HashMap<ObjectId, BlockObject>,
    rects: RectsBatch,
}

struct BlockObject {
    rect: usize,
}

impl BlockPainter {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            indexes: HashMap::new(),
            rects: RectsBatch::new(&renderer),
        }
    }

    pub fn render<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, renderer: &'a Renderer) {
        self.rects.render(render_pass, renderer);
    }

    pub fn insert(&mut self, id: ObjectId, block: BlockBuilder) {
        let rect = self.rects.insert(
            id,
            RectInstance {
                position: block.position.into(),
                color: 1.0, //[255, 255, 255, 255], //block.color.into(),
            },
        );

        self.indexes.insert(id, BlockObject { rect });
    }
}

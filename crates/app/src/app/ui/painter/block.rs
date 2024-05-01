use self::ui::BlockBuilder;
use crate::*;
use digolog_math::*;
use std::collections::HashMap;
use std::simd::num::SimdInt;

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

    pub fn update_buffers(&mut self, encoder: &mut RendererEncoder, renderer: &mut Renderer) {
        self.rects.update_buffers(encoder, renderer);
    }

    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        renderer: &'a mut Renderer,
        camera: &'a Camera2d,
    ) {
        self.rects.render(render_pass, renderer, camera);
    }

    pub fn insert(&mut self, id: ObjectId, block: BlockBuilder) {
        let rect = self.rects.insert(
            id,
            RectInstance {
                position: block.position.cast(),
                size: block.size,
                color: *block.color,
            },
        );

        self.indexes.insert(id, BlockObject { rect });
    }
}

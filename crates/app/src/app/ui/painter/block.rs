use crate::*;
use std::collections::HashMap;

pub struct BlockPainter {
    indexes: HashMap<ObjectId, BlockObject>,
    rects: RectsBatch,
}

struct BlockObject {
    rect: usize,
    pins: Vec<usize>,
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
}
